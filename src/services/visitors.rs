use std::{
    fs::{self, File},
    path::Path,
    sync::Arc,
};

use indexmap::IndexMap;
use log::info;
use openapiv3::{
    AnySchema, ArrayType, AuthorizationCodeOAuth2Flow, BooleanType, Callback,
    ClientCredentialsOAuth2Flow, Components, Contact, Discriminator, Encoding, Example,
    ExternalDocumentation, Header, ImplicitOAuth2Flow, Info, IntegerType, License, Link, MediaType,
    NumberType, OAuth2Flows, ObjectType, OpenAPI, Operation, Parameter, ParameterData,
    ParameterSchemaOrContent, PasswordOAuth2Flow, PathItem, Paths, ReferenceOr, RequestBody,
    Response, Responses, Schema, SecurityRequirement, SecurityScheme, Server, StringType, Tag,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    enums::common::Script,
    holders::context::CACHE,
    services::{comparators::assert_diff, references},
    structs::common::{BracketScripts, CallStack, ParsedSpec},
    traits::common::AsSchemaRef,
    Commands,
};
use anyhow::{anyhow, Context};
use anyhow::{Ok, Result};

use super::cli;

pub fn visit_command(command: &Commands) -> Result<()> {
    if let Commands::Translate {
        spec: spec_path,
        out: out_path,
        clean,
        expected,
        ..
    } = command
    {
        info!("Command execution start for [{:?}]", spec_path);
        info!("Output path [{:?}]", out_path);

        CACHE
            .lock()
            .expect("Could not lock cache for clean")
            .clear();

        if *clean {
            fs::read_dir(out_path)
                .with_context(|| format!("Could not found directory for clean [{:?}]", &out_path))?
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|path| path.is_file())
                .try_for_each(fs::remove_file)
                .with_context(|| format!("Could not clean old code [{:?}]", &out_path))?;
        }

        let spec_as_json: serde_json::Value = serde_yaml::from_reader(
            File::open(spec_path)
                .with_context(|| format!("Could not open spec [{:?}]", &spec_path))?,
        )
        .with_context(|| format!("Could not parse spec as yaml [{:?}]", &spec_path))?;

        let openapi: OpenAPI = serde_json::from_value(spec_as_json.clone())
            .with_context(|| format!("Could not parse spec as OpenAPI v3 [{:?}]", &spec_path))?;

        let call_stack = &cli::set_global_lua_parameters(&openapi)?;

        let parsed_spec = ParsedSpec {
            path: spec_path.to_owned(),
            spec: Arc::new(spec_as_json),
        };

        Script::VisitSpecStart
            .call_with_descriptor(
                out_path,
                &(&openapi.openapi, &openapi.extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_spec_info(out_path, &openapi.info, call_stack)?;
                visit_servers(out_path, &openapi.servers, &openapi.extensions, call_stack)?;
                visit_paths(&parsed_spec, out_path, &openapi.paths, call_stack)?;
                visit_security_requirements(
                    out_path,
                    &openapi.security,
                    &openapi.extensions,
                    call_stack,
                )?;
                visit_spec_tags(out_path, &openapi.tags, &openapi.extensions, call_stack)?;
                visit_external_docs(out_path, &openapi.external_docs, call_stack)?;
                visit_spec_components(&parsed_spec, out_path, &openapi.components, call_stack)?;
                Ok(())
            })?
            .and_then(|call_stack| {
                Script::VisitSpecEnd.call_with_descriptor(
                    out_path,
                    &(&openapi.openapi, &openapi.extensions),
                    call_stack,
                )?;
                Ok(())
            })?;

        if let Some(expected_path) = expected {
            assert_diff(out_path, expected_path)?
        }
        info!("Command execution end for [{:?}]", spec_path);
        Ok(())
    } else {
        Err(anyhow!("Expected a Translate command"))
    }
}

pub fn visit_not<T>(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    schema_ref: &ReferenceOr<T>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()>
where
    T: DeserializeOwned + Send + Sync + AsSchemaRef + From<Schema> + 'static + Serialize,
{
    Script::VisitPropertyNotStart
        .call_with_descriptor(out_path, &(schema_ref, extensions), call_stack)?
        .and_then(|it| visit_schema(parsed_spec, out_path, None, schema_ref, it))?
        .and_then(|call_stack| {
            Script::VisitPropertyNotEnd.call_with_descriptor(
                out_path,
                &(schema_ref, extensions),
                call_stack,
            )?;
            Ok(())
        })?;
    Ok(())
}

pub fn visit_schema<T>(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    schema_name: Option<&str>,
    schema_ref: &ReferenceOr<T>,
    call_stack: &CallStack,
) -> Result<()>
where
    T: DeserializeOwned + Send + Sync + AsSchemaRef + From<Schema> + 'static,
{
    match schema_ref {
        ReferenceOr::Reference { reference } => {
            visit_schema(
                parsed_spec,
                out_path,
                schema_name,
                references::resolve_reference::<T>(reference, parsed_spec)?,
                call_stack,
            )?;
        }
        ReferenceOr::Item(schema_item) => {
            let schema_extensions = &schema_item.as_schema().schema_data.extensions;

            let schema_data = &schema_item.as_schema().schema_data;

            Script::VisitSchemaStart
                .call_with_descriptor(
                    out_path,
                    &(schema_name, &schema_data, &schema_extensions),
                    call_stack,
                )?
                .and_then(|it| {
                    visit_discriminator(out_path, &schema_data.discriminator, it)?;
                    visit_external_docs(out_path, &schema_data.external_docs, call_stack)?;
                    visit_generic_example(
                        out_path,
                        &schema_data.example,
                        &schema_data.extensions,
                        it,
                    )?;
                    visit_schema_default(
                        out_path,
                        &schema_data.default,
                        &schema_data.extensions,
                        it,
                    )?;

                    match &schema_item.as_schema().schema_kind {
                        openapiv3::SchemaKind::Type(type_) => match type_ {
                            openapiv3::Type::Object(object_descriptor) => visit_object(
                                parsed_spec,
                                out_path,
                                object_descriptor,
                                schema_extensions,
                                it,
                            ),
                            openapiv3::Type::Array(array_descriptor) => visit_array(
                                parsed_spec,
                                out_path,
                                array_descriptor,
                                schema_extensions,
                                it,
                            ),
                            // Simple types
                            openapiv3::Type::String(string_descriptor) => {
                                visit_string(out_path, string_descriptor, schema_extensions, it)
                            }
                            openapiv3::Type::Number(number_descriptor) => {
                                visit_number(out_path, number_descriptor, schema_extensions, it)
                            }
                            openapiv3::Type::Integer(integer_descriptor) => {
                                visit_integer(out_path, integer_descriptor, schema_extensions, it)
                            }
                            openapiv3::Type::Boolean(boolean_descriptor) => {
                                visit_boolean(out_path, boolean_descriptor, schema_extensions, it)
                            }
                        },
                        openapiv3::SchemaKind::OneOf { one_of } => {
                            visit_one_of(parsed_spec, out_path, one_of, schema_extensions, it)
                        }
                        openapiv3::SchemaKind::AllOf { all_of } => {
                            visit_all_of(parsed_spec, out_path, all_of, schema_extensions, it)
                        }
                        openapiv3::SchemaKind::AnyOf { any_of } => {
                            visit_any_of(parsed_spec, out_path, any_of, schema_extensions, it)
                        }
                        openapiv3::SchemaKind::Not { not } => {
                            visit_not(parsed_spec, out_path, not, schema_extensions, it)
                        }
                        openapiv3::SchemaKind::Any(any_schema) => {
                            visit_any_schema(out_path, any_schema, schema_extensions, it)
                        }
                    }
                })?
                .and_then(|call_stack| {
                    Script::VisitSchemaEnd.call_with_descriptor(
                        out_path,
                        &(schema_name, schema_data, schema_extensions),
                        call_stack,
                    )?;
                    Ok(())
                })?;
        }
    }
    Ok(())
}

pub fn visit_response(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    response_name: Option<&str>,
    response_ref: &ReferenceOr<Response>,
    call_stack: &CallStack,
) -> Result<()> {
    match response_ref {
        ReferenceOr::Reference { reference } => {
            visit_response(
                parsed_spec,
                out_path,
                response_name,
                references::resolve_reference::<Response>(reference, parsed_spec)?,
                call_stack,
            )?;
        }
        ReferenceOr::Item(response) => {
            let response_extensions = &response.extensions;

            Script::VisitResponseStart
                .call_with_descriptor(
                    out_path,
                    &(&response_name, response, &response_extensions),
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_headers(
                        parsed_spec,
                        out_path,
                        &response.headers,
                        response_extensions,
                        call_stack,
                    )?;

                    visit_media_types(
                        parsed_spec,
                        out_path,
                        &response.content,
                        response_extensions,
                        call_stack,
                    )?;

                    visit_links(
                        parsed_spec,
                        out_path,
                        &response.links,
                        response_extensions,
                        call_stack,
                    )?;
                    Ok(())
                })?
                .and_then(|call_stack| {
                    Script::VisitResponseEnd.call_with_descriptor(
                        out_path,
                        &(response_name, response, response_extensions),
                        call_stack,
                    )?;
                    Ok(())
                })?;
        }
    }
    Ok(())
}

pub fn visit_string(
    out_path: &Path,
    string_descriptor: &StringType,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    Script::VisitStringProperty.call_with_descriptor(
        out_path,
        &(string_descriptor, extensions),
        call_stack,
    )?;
    Ok(())
}

pub fn visit_any_schema(
    out_path: &Path,
    any_schema_descriptor: &AnySchema,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    Script::VisitAnySchema.call_with_descriptor(
        out_path,
        &(any_schema_descriptor, extensions),
        call_stack,
    )?;
    Ok(())
}

pub fn visit_number(
    out_path: &Path,
    number_descriptor: &NumberType,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    Script::VisitNumberProperty.call_with_descriptor(
        out_path,
        &(number_descriptor, extensions),
        call_stack,
    )?;
    Ok(())
}

pub fn visit_integer(
    out_path: &Path,
    integer_descriptor: &IntegerType,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    Script::VisitIntegerProperty.call_with_descriptor(
        out_path,
        &(integer_descriptor, extensions),
        call_stack,
    )?;
    Ok(())
}

pub fn visit_array(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    array_descriptor: &ArrayType,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    Script::VisitArrayPropertyStart
        .call_with_descriptor(
            out_path,
            &(array_descriptor, extensions, call_stack),
            call_stack,
        )?
        .and_then(|call_stack| {
            if let Some(it) = &array_descriptor.items {
                visit_schema(parsed_spec, out_path, None, it, call_stack)?;
            }
            Ok(())
        })?
        .and_then(|call_stack| {
            Script::VisitArrayPropertyEnd.call_with_descriptor(
                out_path,
                &(array_descriptor, extensions),
                call_stack,
            )?;
            Ok(())
        })?;
    Ok(())
}

pub fn visit_boolean(
    out_path: &Path,
    boolean_descriptor: &BooleanType,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    Script::VisitBooleanProperty.call_with_descriptor(
        out_path,
        &(boolean_descriptor, extensions),
        call_stack,
    )?;
    Ok(())
}

pub fn visit_one_of(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    schemas: &[ReferenceOr<Schema>],
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !schemas.is_empty() {
        Script::VisitOneOfStart
            .call_with_descriptor(out_path, &(schemas, &extensions), call_stack)?
            .and_then(|call_stack| {
                schemas
                    .iter()
                    .try_for_each(|it| visit_schema(parsed_spec, out_path, None, it, call_stack))
            })?
            .and_then(|call_stack| {
                Script::VisitOneOfEnd.call_with_descriptor(
                    out_path,
                    &(schemas, extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_all_of(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    schemas: &[ReferenceOr<Schema>],
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !schemas.is_empty() {
        Script::VisitAllOfStart
            .call_with_descriptor(out_path, &(schemas, extensions), call_stack)?
            .and_then(|call_stack| {
                schemas
                    .iter()
                    .try_for_each(|it| visit_schema(parsed_spec, out_path, None, it, call_stack))
            })?
            .and_then(|call_stack| {
                Script::VisitAllOfEnd.call_with_descriptor(
                    out_path,
                    &(schemas, extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_any_of(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    schemas: &[ReferenceOr<Schema>],
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !schemas.is_empty() {
        Script::VisitAnyOfStart
            .call_with_descriptor(out_path, &(schemas, extensions), call_stack)?
            .and_then(|call_stack| {
                schemas
                    .iter()
                    .try_for_each(|it| visit_schema(parsed_spec, out_path, None, it, call_stack))
            })?
            .and_then(|call_stack| {
                Script::VisitAnyOfEnd.call_with_descriptor(
                    out_path,
                    &(schemas, extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_discriminator(
    out_path: &Path,
    dicriminator: &Option<Discriminator>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(discriminator) = dicriminator.as_ref() {
        Script::VisitDiscriminator.call_with_descriptor(
            out_path,
            &(discriminator, &discriminator.extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_generic_example(
    out_path: &Path,
    example: &Option<serde_json::Value>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(example) = example {
        Script::VisitGenericExample.call_with_descriptor(
            out_path,
            &(&example, extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_generic_parameter(
    out_path: &Path,
    parameter_name: &str,
    parameter: &serde_json::Value,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    Script::VisitGenericParameter.call_with_descriptor(
        out_path,
        &(parameter_name, parameter, extensions),
        call_stack,
    )?;
    Ok(())
}

pub fn visit_generic_request_body(
    out_path: &Path,
    body: &Option<serde_json::Value>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(body_json) = body {
        Script::VisitGenericRequestBody.call_with_descriptor(
            out_path,
            &(&body_json, extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_media_type_encodings(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    encodings: &IndexMap<String, Encoding>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !encodings.is_empty() {
        Script::VisitEncodingsStart
            .call_with_descriptor(out_path, &(&encodings, extensions), call_stack)?
            .and_then(|call_stack| {
                encodings.iter().try_for_each(|encoding| {
                    visit_media_type_encoding(
                        parsed_spec,
                        out_path,
                        encoding.0,
                        encoding.1,
                        call_stack,
                    )
                })
            })?
            .and_then(|call_stack| {
                Script::VisitEncodingsEnd.call_with_descriptor(
                    out_path,
                    &(&encodings, extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_media_type_encoding(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    encoding_name: &str,
    encoding: &Encoding,
    call_stack: &CallStack,
) -> Result<()> {
    Script::VisitEncodingStart
        .call_with_descriptor(
            out_path,
            &(&encoding_name, &encoding, &encoding.extensions),
            call_stack,
        )?
        .and_then(|call_stack| {
            visit_headers(
                parsed_spec,
                out_path,
                &encoding.headers,
                &encoding.extensions,
                call_stack,
            )
        })?
        .and_then(|call_stack| {
            Script::VisitEncodingEnd.call_with_descriptor(
                out_path,
                &(&encoding_name, &encoding, &encoding.extensions),
                call_stack,
            )?;
            Ok(())
        })?;
    Ok(())
}

pub fn visit_example(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    example_name: &str,
    example_ref: &ReferenceOr<Example>,
    call_stack: &CallStack,
) -> Result<()> {
    match example_ref {
        ReferenceOr::Reference { reference } => {
            visit_example(
                parsed_spec,
                out_path,
                example_name,
                references::resolve_reference::<Example>(reference, parsed_spec)?,
                call_stack,
            )?;
        }
        ReferenceOr::Item(example) => {
            Script::VisitExampleStart
                .call_with_descriptor(
                    out_path,
                    &(example_name, &example, &example.extensions),
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_generic_example(out_path, &example.value, &example.extensions, call_stack)
                })?
                .and_then(|call_stack| {
                    Script::VisitExampleEnd.call_with_descriptor(
                        out_path,
                        &(example_name, &example, &example.extensions),
                        call_stack,
                    )?;
                    Ok(())
                })?;
        }
    }
    Ok(())
}

pub fn visit_request_body(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    request_body_name: Option<&str>,
    request_body_ref: &ReferenceOr<RequestBody>,
    call_stack: &CallStack,
) -> Result<()> {
    match request_body_ref {
        ReferenceOr::Reference { reference } => {
            visit_request_body(
                parsed_spec,
                out_path,
                request_body_name,
                references::resolve_reference::<RequestBody>(reference, parsed_spec)?,
                call_stack,
            )?;
        }
        ReferenceOr::Item(request_body) => {
            Script::VisitRequestBodyStart
                .call_with_descriptor(
                    out_path,
                    &(&request_body_name, &request_body, &request_body.extensions),
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_media_types(
                        parsed_spec,
                        out_path,
                        &request_body.content,
                        &request_body.extensions,
                        call_stack,
                    )
                })?
                .and_then(|call_stack| {
                    Script::VisitRequestBodyEnd.call_with_descriptor(
                        out_path,
                        &(&request_body_name, &request_body, &request_body.extensions),
                        call_stack,
                    )?;
                    Ok(())
                })?;
        }
    }
    Ok(())
}

pub fn visit_parameter_schema_or_content(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameter_name: Option<&str>,
    parameter_schema_or_content: &ParameterSchemaOrContent,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    Script::VisitParameterSchemaOrContentStart
        .call_with_descriptor(
            out_path,
            &(&parameter_name, &parameter_schema_or_content, extensions),
            call_stack,
        )?
        .and_then(|call_stack| {
            match parameter_schema_or_content {
                ParameterSchemaOrContent::Schema(schema_ref) => {
                    visit_schema(
                        parsed_spec,
                        out_path,
                        parameter_name,
                        schema_ref,
                        call_stack,
                    )?;
                }
                ParameterSchemaOrContent::Content(media_types) => {
                    visit_media_types(parsed_spec, out_path, media_types, extensions, call_stack)?;
                }
            }
            Ok(())
        })?
        .and_then(|call_stack| {
            Script::VisitParameterSchemaOrContentEnd.call_with_descriptor(
                out_path,
                &(&parameter_name, &parameter_schema_or_content, extensions),
                call_stack,
            )?;
            Ok(())
        })?;
    Ok(())
}

pub fn visit_media_types(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    media_types: &IndexMap<String, MediaType>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !media_types.is_empty() {
        Script::VisitMediaTypesStart
            .call_with_descriptor(out_path, &(media_types, extensions), call_stack)?
            .and_then(|call_stack| {
                media_types.iter().try_for_each(|media_type| {
                    visit_media_type(
                        parsed_spec,
                        out_path,
                        media_type.0,
                        media_type.1,
                        call_stack,
                    )
                })
            })?
            .and_then(|call_stack| {
                Script::VisitMediaTypesEnd.call_with_descriptor(
                    out_path,
                    &(media_types, extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_callbacks(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    operation_callbacks: &IndexMap<String, ReferenceOr<Callback>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !operation_callbacks.is_empty() {
        Script::VisitAsyncCallbacksStart
            .call_with_descriptor(out_path, &(operation_callbacks, &extensions), call_stack)?
            .and_then(|call_stack| {
                operation_callbacks.iter().try_for_each(|callbacks| {
                    visit_callback(
                        parsed_spec,
                        out_path,
                        callbacks.0,
                        callbacks.1,
                        extensions,
                        call_stack,
                    )
                })
            })?
            .and_then(|call_stack| {
                Script::VisitAsyncCallbacksEnd.call_with_descriptor(
                    out_path,
                    &(operation_callbacks, &extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_links(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    links: &IndexMap<String, ReferenceOr<Link>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !links.is_empty() {
        Script::VisitLinksStart
            .call_with_descriptor(out_path, &(links, &extensions), call_stack)?
            .and_then(|call_stack| {
                links.iter().try_for_each(|link| {
                    visit_link(parsed_spec, out_path, link.0, link.1, call_stack)
                })
            })?
            .and_then(|call_stack| {
                Script::VisitLinksEnd.call_with_descriptor(
                    out_path,
                    &(links, &extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_link(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    link_name: &str,
    link: &ReferenceOr<Link>,
    call_stack: &CallStack,
) -> Result<()> {
    match link {
        ReferenceOr::Reference { reference } => {
            visit_link(
                parsed_spec,
                out_path,
                link_name,
                references::resolve_reference::<Link>(reference, parsed_spec)?,
                call_stack,
            )?;
        }
        ReferenceOr::Item(link) => {
            Script::VisitLinkStart
                .call_with_descriptor(out_path, &(link_name, link, &link.extensions), call_stack)?
                .and_then(|call_stack| {
                    visit_generic_request_body(
                        out_path,
                        &link.request_body,
                        &link.extensions,
                        call_stack,
                    )?;

                    visit_generic_parameters(
                        out_path,
                        &link.parameters,
                        &link.extensions,
                        call_stack,
                    )?;

                    if let Some(server) = &link.server {
                        visit_server(out_path, server, call_stack)?;
                    }
                    Ok(())
                })?
                .and_then(|call_stack| {
                    Script::VisitLinkEnd.call_with_descriptor(
                        out_path,
                        &(link_name, link, &link.extensions),
                        call_stack,
                    )?;
                    Ok(())
                })?;
        }
    }
    Ok(())
}

pub fn visit_callback(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    callbacks_name: &str,
    callbacks: &ReferenceOr<Callback>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    match callbacks {
        ReferenceOr::Reference { reference } => {
            visit_callback(
                parsed_spec,
                out_path,
                callbacks_name,
                references::resolve_reference::<Callback>(reference, parsed_spec)?,
                extensions,
                call_stack,
            )?;
        }
        ReferenceOr::Item(callback) => {
            Script::VisitAsyncCallbackStart
                .call_with_descriptor(
                    out_path,
                    &(callbacks_name, callback, &extensions),
                    call_stack,
                )?
                .and_then(|call_stack| {
                    callback.iter().try_for_each(|it| {
                        visit_path_item(parsed_spec, out_path, it.0, it.1, call_stack)
                    })
                })?
                .and_then(|call_stack| {
                    Script::VisitAsyncCallbackEnd.call_with_descriptor(
                        out_path,
                        &(callbacks_name, callback, &extensions),
                        call_stack,
                    )?;
                    Ok(())
                })?;
        }
    }
    Ok(())
}

pub fn visit_media_type(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    media_type_name: &str,
    media_type: &MediaType,
    call_stack: &CallStack,
) -> Result<()> {
    Script::VisitMediaTypeStart
        .call_with_descriptor(
            out_path,
            &(&media_type_name, media_type, &media_type.extensions),
            call_stack,
        )?
        .and_then(|call_stack| {
            if let Some(schema_ref) = &media_type.schema {
                visit_schema(parsed_spec, out_path, None, schema_ref, call_stack)?;
            }

            visit_generic_example(
                out_path,
                &media_type.example,
                &media_type.extensions,
                call_stack,
            )?;

            visit_examples(
                parsed_spec,
                out_path,
                &media_type.examples,
                &media_type.extensions,
                call_stack,
            )?;

            visit_media_type_encodings(
                parsed_spec,
                out_path,
                &media_type.encoding,
                &media_type.extensions,
                call_stack,
            )?;
            Ok(())
        })?
        .and_then(|call_stack| {
            Script::VisitMediaTypeEnd.call_with_descriptor(
                out_path,
                &(&media_type_name, media_type, &media_type.extensions),
                call_stack,
            )?;
            Ok(())
        })?;
    Ok(())
}

pub fn visit_examples(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    examples: &IndexMap<String, ReferenceOr<Example>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !examples.is_empty() {
        Script::VisitExamplesStart
            .call_with_descriptor(out_path, &(&examples, extensions), call_stack)?
            .and_then(|call_stack| {
                examples
                    .iter()
                    .try_for_each(|it| visit_example(parsed_spec, out_path, it.0, it.1, call_stack))
            })?
            .and_then(|call_stack| {
                Script::VisitExamplesEnd.call_with_descriptor(
                    out_path,
                    &(&examples, extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_request_bodies(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    request_bodies: &IndexMap<String, ReferenceOr<RequestBody>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !request_bodies.is_empty() {
        Script::VisitRequestBodiesStart
            .call_with_descriptor(out_path, &(request_bodies, extensions), call_stack)?
            .and_then(|call_stack| {
                request_bodies.iter().try_for_each(|it| {
                    visit_request_body(parsed_spec, out_path, Some(it.0), it.1, call_stack)
                })
            })?
            .and_then(|call_stack| {
                Script::VisitRequestBodiesEnd.call_with_descriptor(
                    out_path,
                    &(request_bodies, extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_generic_parameters(
    out_path: &Path,
    parameters: &IndexMap<String, serde_json::Value>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !parameters.is_empty() {
        Script::VisitGenericParametersStart
            .call_with_descriptor(out_path, &(&parameters, extensions), call_stack)?
            .and_then(|call_stack| {
                parameters.iter().try_for_each(|it| {
                    visit_generic_parameter(out_path, it.0, it.1, extensions, call_stack)?;
                    Ok(())
                })
            })?
            .and_then(|call_stack| {
                Script::VisitGenericParametersEnd.call_with_descriptor(
                    out_path,
                    &(&parameters, extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_header(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    header_name: &str,
    header: &ReferenceOr<Header>,
    call_stack: &CallStack,
) -> Result<()> {
    match header {
        ReferenceOr::Reference { reference } => {
            visit_header(
                parsed_spec,
                out_path,
                header_name,
                references::resolve_reference::<Header>(reference, parsed_spec)?,
                call_stack,
            )?;
        }
        ReferenceOr::Item(header) => {
            Script::VisitHeaderStart
                .call_with_descriptor(
                    out_path,
                    &(&header_name, &header, &header.extensions),
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_parameter_schema_or_content(
                        parsed_spec,
                        out_path,
                        None,
                        &header.format,
                        &header.extensions,
                        call_stack,
                    )?;

                    visit_generic_example(
                        out_path,
                        &header.example,
                        &header.extensions,
                        call_stack,
                    )?;

                    visit_examples(
                        parsed_spec,
                        out_path,
                        &header.examples,
                        &header.extensions,
                        call_stack,
                    )
                })?
                .and_then(|call_stack| {
                    Script::VisitHeaderEnd.call_with_descriptor(
                        out_path,
                        &(&header_name, &header, &header.extensions),
                        call_stack,
                    )?;
                    Ok(())
                })?;
        }
    }
    Ok(())
}

pub fn visit_security_scheme(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    scheme_name: &str,
    security_scheme: &ReferenceOr<SecurityScheme>,
    call_stack: &CallStack,
) -> Result<()> {
    match security_scheme {
        ReferenceOr::Reference { reference } => {
            visit_security_scheme(
                parsed_spec,
                out_path,
                scheme_name,
                references::resolve_reference::<SecurityScheme>(reference, parsed_spec)?,
                call_stack,
            )?;
        }
        ReferenceOr::Item(security_scheme) => match security_scheme {
            SecurityScheme::APIKey { .. } => {
                visit_security_scheme_apikey(out_path, scheme_name, security_scheme, call_stack)?;
            }
            SecurityScheme::HTTP { .. } => {
                visit_security_scheme_http(out_path, scheme_name, security_scheme, call_stack)?;
            }
            SecurityScheme::OAuth2 { .. } => {
                visit_security_scheme_oauth2(out_path, scheme_name, security_scheme, call_stack)?;
            }
            SecurityScheme::OpenIDConnect { .. } => {
                visit_security_scheme_openid_connect(
                    out_path,
                    scheme_name,
                    security_scheme,
                    call_stack,
                )?;
            }
        },
    }
    Ok(())
}

pub fn visit_headers(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    headers: &IndexMap<String, ReferenceOr<Header>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !headers.is_empty() {
        Script::VisitHeadersStart
            .call_with_descriptor(out_path, &(headers, extensions), call_stack)?
            .and_then(|call_stack| {
                headers
                    .iter()
                    .try_for_each(|it| visit_header(parsed_spec, out_path, it.0, it.1, call_stack))
            })?
            .and_then(|call_stack| {
                Script::VisitHeadersEnd.call_with_descriptor(
                    out_path,
                    &(headers, extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_security_schemes(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    security_schemes: &IndexMap<String, ReferenceOr<SecurityScheme>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !security_schemes.is_empty() {
        Script::VisitSecuritySchemesStart
            .call_with_descriptor(out_path, &(&security_schemes, extensions), call_stack)?
            .and_then(|call_stack| {
                security_schemes.iter().try_for_each(|it| {
                    visit_security_scheme(parsed_spec, out_path, it.0, it.1, call_stack)
                })
            })?
            .and_then(|call_stack| {
                Script::VisitSecuritySchemesEnd.call_with_descriptor(
                    out_path,
                    &(&security_schemes, extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_spec_tags(
    out_path: &Path,
    tags: &Vec<Tag>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !tags.is_empty() {
        Script::VisitSpecTagsStart
            .call_with_descriptor(out_path, &(tags, extensions), call_stack)?
            .and_then(|call_stack| {
                tags.iter().try_for_each(|tag| {
                    visit_external_docs(out_path, &tag.external_docs, call_stack)?;
                    Script::VisitSpecTag.call_with_descriptor(
                        out_path,
                        &(tag, &tag.extensions),
                        call_stack,
                    )?;
                    Ok(())
                })
            })?
            .and_then(|call_stack| {
                Script::VisitSpecTagsEnd.call_with_descriptor(
                    out_path,
                    &(tags, extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_security_requirements(
    out_path: &Path,
    securities: &Option<Vec<SecurityRequirement>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(it) = securities.as_ref() {
        if !it.is_empty() {
            Script::VisitSecurityRequirementsStart
                .call_with_descriptor(out_path, &(it, extensions), call_stack)?
                .and_then(|call_stack| {
                    it.iter().try_for_each(|sec_map| {
                        Script::VisitSecurityRequirement.call_with_descriptor(
                            out_path,
                            &(sec_map, extensions),
                            call_stack,
                        )?;
                        Ok(())
                    })
                })?
                .and_then(|call_stack| {
                    Script::VisitSecurityRequirementsEnd.call_with_descriptor(
                        out_path,
                        &(it, extensions),
                        call_stack,
                    )?;
                    Ok(())
                })?;
        }
    }
    Ok(())
}
pub fn visit_external_docs(
    out_path: &Path,
    external_docs: &Option<ExternalDocumentation>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(it) = external_docs {
        Script::VisitExternalDocs.call_with_descriptor(
            out_path,
            &(it, &it.extensions),
            call_stack,
        )?;
    }
    Ok(())
}
pub fn visit_schemas(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    schemas: &IndexMap<String, ReferenceOr<Schema>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !schemas.is_empty() {
        Script::VisitSchemasStart
            .call_with_descriptor(out_path, &(schemas, extensions), call_stack)?
            .and_then(|it| {
                schemas.iter().try_for_each(|(schema_name, schema_ref)| {
                    visit_schema(parsed_spec, out_path, Some(schema_name), schema_ref, it)
                })
            })?
            .and_then(|call_stack| {
                Script::VisitSchemasEnd.call_with_descriptor(
                    out_path,
                    &(schemas, extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_components_responses(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    responses: &IndexMap<String, ReferenceOr<Response>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !responses.is_empty() {
        Script::VisitResponsesStart
            .call_with_descriptor(out_path, &(responses, extensions), call_stack)?
            .and_then(|it| {
                responses
                    .iter()
                    .try_for_each(|(response_name, response_ref)| {
                        visit_response(parsed_spec, out_path, Some(response_name), response_ref, it)
                    })
            })?
            .and_then(|call_stack| {
                Script::VisitResponsesEnd.call_with_descriptor(
                    out_path,
                    &(responses, extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_operation_responses(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    responses: &Responses,
    call_stack: &CallStack,
) -> Result<()> {
    Script::VisitOperationResponsesStart
        .call_with_descriptor(out_path, &(&responses, &responses.extensions), call_stack)?
        .and_then(|call_stack| {
            if let Some(response) = &responses.default {
                visit_response(parsed_spec, out_path, None, response, call_stack)?;
            }

            responses
                .responses
                .iter()
                .try_for_each(|(status, response_ref)| {
                    visit_response(
                        parsed_spec,
                        out_path,
                        Some(&status.to_string()),
                        response_ref,
                        call_stack,
                    )
                })
        })?
        .and_then(|call_stack| {
            Script::VisitOperationResponsesEnd.call_with_descriptor(
                out_path,
                &(&responses, &responses.extensions),
                call_stack,
            )?;
            Ok(())
        })?;
    Ok(())
}

pub fn visit_parameters(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameters: &IndexMap<String, ReferenceOr<Parameter>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !parameters.is_empty() {
        Script::VisitParametersStart
            .call_with_descriptor(out_path, &(parameters, &extensions), call_stack)?
            .and_then(|call_stack| {
                parameters
                    .iter()
                    .try_for_each(|(parameter_name, parameter_ref)| {
                        visit_parameter(
                            parsed_spec,
                            out_path,
                            parameter_name,
                            parameter_ref,
                            extensions,
                            call_stack,
                        )?;
                        Ok(())
                    })
            })?
            .and_then(|call_stack| {
                Script::VisitParametersEnd.call_with_descriptor(
                    out_path,
                    &(parameters, &extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_paths(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    paths: &Paths,
    call_stack: &CallStack,
) -> Result<()> {
    if !paths.paths.is_empty() {
        Script::VisitPathsStart
            .call_with_descriptor(out_path, &(&paths, &paths.extensions), call_stack)?
            .and_then(|call_stack| {
                paths.paths.iter().try_for_each(|it| {
                    visit_path_item_ref(parsed_spec, out_path, it.0, it.1, call_stack)
                })
            })?
            .and_then(|call_stack| {
                Script::VisitPathsEnd.call_with_descriptor(
                    out_path,
                    &(&paths, &paths.extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_path_item_ref(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    path_item_name: &str,
    path_item_ref: &ReferenceOr<PathItem>,
    call_stack: &CallStack,
) -> Result<()> {
    match path_item_ref {
        ReferenceOr::Reference { reference } => {
            visit_path_item_ref(
                parsed_spec,
                out_path,
                path_item_name,
                references::resolve_reference::<PathItem>(reference, parsed_spec)?,
                call_stack,
            )?;
        }
        ReferenceOr::Item(path_item) => {
            visit_path_item(parsed_spec, out_path, path_item_name, path_item, call_stack)?;
        }
    }
    Ok(())
}

pub fn visit_parameter(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameter_name: &str,
    parameter_ref: &ReferenceOr<Parameter>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    match parameter_ref {
        ReferenceOr::Reference { reference } => {
            visit_parameter(
                parsed_spec,
                out_path,
                parameter_name,
                references::resolve_reference::<Parameter>(reference, parsed_spec)?,
                extensions,
                call_stack,
            )?;
        }
        ReferenceOr::Item(parameter) => match parameter {
            Parameter::Query { .. } => {
                visit_query_parameter(
                    parsed_spec,
                    out_path,
                    parameter_name,
                    parameter,
                    extensions,
                    call_stack,
                )?;
            }
            Parameter::Header { .. } => {
                visit_header_parameter(
                    parsed_spec,
                    out_path,
                    parameter_name,
                    parameter,
                    extensions,
                    call_stack,
                )?;
            }
            Parameter::Path { .. } => {
                visit_path_parameter(
                    parsed_spec,
                    out_path,
                    parameter_name,
                    parameter,
                    extensions,
                    call_stack,
                )?;
            }
            Parameter::Cookie { .. } => {
                visit_cookie_parameter(
                    parsed_spec,
                    out_path,
                    parameter_name,
                    parameter,
                    extensions,
                    call_stack,
                )?;
            }
        },
    }
    Ok(())
}

pub fn visit_security_scheme_http(
    out_path: &Path,
    scheme_name: &str,
    http: &SecurityScheme,
    call_stack: &CallStack,
) -> Result<()> {
    if let SecurityScheme::HTTP { extensions, .. } = http {
        Script::VisitSecuritySchemeHttp.call_with_descriptor(
            out_path,
            &(&scheme_name, &http, &extensions),
            call_stack,
        )?;
        Ok(())
    } else {
        Err(anyhow!("Not a SecurityScheme.HTTP"))
    }
}

pub fn visit_security_scheme_oauth2(
    out_path: &Path,
    scheme_name: &str,
    oauth2: &SecurityScheme,
    call_stack: &CallStack,
) -> Result<()> {
    if let SecurityScheme::OAuth2 {
        flows, extensions, ..
    } = oauth2
    {
        Script::VisitSecuritySchemeOAuth2Start
            .call_with_descriptor(out_path, &(&scheme_name, &oauth2, extensions), call_stack)?
            .and_then(|call_stack| visit_security_scheme_oauth2_flows(out_path, flows, call_stack))?
            .and_then(|call_stack| {
                Script::VisitSecuritySchemeOAuth2End.call_with_descriptor(
                    out_path,
                    &(&scheme_name, &oauth2, extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
        Ok(())
    } else {
        Err(anyhow!("Not a SecurityScheme.OAuth2"))
    }
}

pub fn visit_security_scheme_oauth2_flows(
    out_path: &Path,
    flows: &OAuth2Flows,
    call_stack: &CallStack,
) -> Result<()> {
    Script::VisitSecuritySchemeOAuth2FlowsStart
        .call_with_descriptor(out_path, &(flows, &flows.extensions), call_stack)?
        .and_then(|call_stack| {
            visit_security_scheme_oauth2_flows_implicit(out_path, &flows.implicit, call_stack)?;
            visit_security_scheme_oauth2_flows_password(out_path, &flows.password, call_stack)?;
            visit_security_scheme_oauth2_flows_client_credentials(
                out_path,
                &flows.client_credentials,
                call_stack,
            )?;
            visit_security_scheme_oauth2_flows_authorization_code(
                out_path,
                &flows.authorization_code,
                call_stack,
            )
        })?
        .and_then(|call_stack| {
            Script::VisitSecuritySchemeOAuth2FlowsEnd.call_with_descriptor(
                out_path,
                &(flows, &flows.extensions),
                call_stack,
            )?;
            Ok(())
        })?;
    Ok(())
}

pub fn visit_security_scheme_oauth2_flows_implicit(
    out_path: &Path,
    flow: &Option<ImplicitOAuth2Flow>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(flow) = flow {
        Script::VisitSecuritySchemeOAuth2FlowImplicit.call_with_descriptor(
            out_path,
            &(flow, &flow.extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_security_scheme_oauth2_flows_password(
    out_path: &Path,
    flow: &Option<PasswordOAuth2Flow>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(flow) = flow {
        Script::VisitSecuritySchemeOAuth2FlowPassword.call_with_descriptor(
            out_path,
            &(flow, &flow.extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_security_scheme_oauth2_flows_client_credentials(
    out_path: &Path,
    flow: &Option<ClientCredentialsOAuth2Flow>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(flow) = flow {
        Script::VisitSecuritySchemeOAuth2FlowClientCredentials.call_with_descriptor(
            out_path,
            &(flow, &flow.extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_security_scheme_oauth2_flows_authorization_code(
    out_path: &Path,
    flow: &Option<AuthorizationCodeOAuth2Flow>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(flow) = flow {
        Script::VisitSecuritySchemeOAuth2FlowAuthorizationCode.call_with_descriptor(
            out_path,
            &(flow, &flow.extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_security_scheme_apikey(
    out_path: &Path,
    scheme_name: &str,
    api_key: &SecurityScheme,
    call_stack: &CallStack,
) -> Result<()> {
    if let SecurityScheme::APIKey { extensions, .. } = api_key {
        Script::VisitSecuritySchemeApiKey.call_with_descriptor(
            out_path,
            &(scheme_name, &api_key, &extensions),
            call_stack,
        )?;
        Ok(())
    } else {
        Err(anyhow!("Not a SecurityScheme.APIKey"))
    }
}

pub fn visit_security_scheme_openid_connect(
    out_path: &Path,
    scheme_name: &str,
    openid_connect: &SecurityScheme,
    call_stack: &CallStack,
) -> Result<()> {
    if let SecurityScheme::OpenIDConnect { extensions, .. } = openid_connect {
        Script::VisitSecuritySchemeOpenIdConnect.call_with_descriptor(
            out_path,
            &(scheme_name, &openid_connect, &extensions),
            call_stack,
        )?;
        Ok(())
    } else {
        Err(anyhow!("Not a SecurityScheme.OpenIDConnect"))
    }
}

pub fn visit_parameter_data(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameter_data: &ParameterData,
    call_stack: &CallStack,
) -> Result<()> {
    Script::VisitParameterDataStart
        .call_with_descriptor(
            out_path,
            &(parameter_data, &parameter_data.extensions),
            call_stack,
        )?
        .and_then(|call_stack| {
            visit_parameter_schema_or_content(
                parsed_spec,
                out_path,
                None,
                &parameter_data.format,
                &parameter_data.extensions,
                call_stack,
            )?;
            visit_generic_example(
                out_path,
                &parameter_data.example,
                &parameter_data.extensions,
                call_stack,
            )?;
            visit_examples(
                parsed_spec,
                out_path,
                &parameter_data.examples,
                &parameter_data.extensions,
                call_stack,
            )?;
            Ok(())
        })?
        .and_then(|call_stack| {
            Script::VisitParameterDataEnd.call_with_descriptor(
                out_path,
                &(parameter_data, &parameter_data.extensions),
                call_stack,
            )?;
            Ok(())
        })?;
    Ok(())
}

pub fn visit_query_parameter(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameter_name: &str,
    parameter: &Parameter,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Parameter::Query { parameter_data, .. } = parameter {
        Script::VisitQueryParameterStart
            .call_with_descriptor(
                out_path,
                &(parameter_name, &parameter, &extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_parameter_data(parsed_spec, out_path, parameter_data, call_stack)?;
                Ok(())
            })?
            .and_then(|call_stack| {
                Script::VisitQueryParameterEnd.call_with_descriptor(
                    out_path,
                    &(parameter_name, &parameter, &extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
        Ok(())
    } else {
        Err(anyhow!("Not a Query parameter"))
    }
}

pub fn visit_header_parameter(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameter_name: &str,
    parameter: &Parameter,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Parameter::Header { parameter_data, .. } = parameter {
        Script::VisitHeaderParameterStart
            .call_with_descriptor(
                out_path,
                &(parameter_name, &parameter, &extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_parameter_data(parsed_spec, out_path, parameter_data, call_stack)?;
                Ok(())
            })?
            .and_then(|call_stack| {
                Script::VisitHeaderParameterEnd.call_with_descriptor(
                    out_path,
                    &(parameter_name, &parameter, &extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
        Ok(())
    } else {
        Err(anyhow!("Not a Header parameter"))
    }
}

pub fn visit_path_parameter(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameter_name: &str,
    parameter: &Parameter,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Parameter::Path { parameter_data, .. } = parameter {
        Script::VisitPathParameterStart
            .call_with_descriptor(
                out_path,
                &(parameter_name, &parameter, &extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_parameter_data(parsed_spec, out_path, parameter_data, call_stack)?;
                Ok(())
            })?
            .and_then(|call_stack| {
                Script::VisitPathParameterEnd.call_with_descriptor(
                    out_path,
                    &(parameter_name, &parameter, &extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
        Ok(())
    } else {
        Err(anyhow!("Not a Path parameter"))
    }
}

pub fn visit_path_item(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    path_item_name: &str,
    path_item: &PathItem,
    call_stack: &CallStack,
) -> Result<()> {
    Script::VisitPathItemStart
        .call_with_descriptor(
            out_path,
            &(path_item_name, &path_item, &path_item.extensions),
            call_stack,
        )?
        .and_then(|call_stack| {
            visit_operation(
                parsed_spec,
                out_path,
                &path_item.trace,
                &BracketScripts {
                    start: Script::VisitTraceOperationStart,
                    end: Script::VisitTraceOperationEnd,
                },
                call_stack,
            )?;
            visit_operation(
                parsed_spec,
                out_path,
                &path_item.put,
                &BracketScripts {
                    start: Script::VisitPutOperationStart,
                    end: Script::VisitPutOperationEnd,
                },
                call_stack,
            )?;
            visit_operation(
                parsed_spec,
                out_path,
                &path_item.post,
                &BracketScripts {
                    start: Script::VisitPostOperationStart,
                    end: Script::VisitPostOperationEnd,
                },
                call_stack,
            )?;
            visit_operation(
                parsed_spec,
                out_path,
                &path_item.patch,
                &BracketScripts {
                    start: Script::VisitPatchOperationStart,
                    end: Script::VisitPatchOperationEnd,
                },
                call_stack,
            )?;
            visit_operation(
                parsed_spec,
                out_path,
                &path_item.options,
                &BracketScripts {
                    start: Script::VisitOptionsOperationStart,
                    end: Script::VisitOptionsOperationEnd,
                },
                call_stack,
            )?;
            visit_operation(
                parsed_spec,
                out_path,
                &path_item.head,
                &BracketScripts {
                    start: Script::VisitHeadOperationStart,
                    end: Script::VisitHeadOperationEnd,
                },
                call_stack,
            )?;
            visit_operation(
                parsed_spec,
                out_path,
                &path_item.get,
                &BracketScripts {
                    start: Script::VisitGetOperationStart,
                    end: Script::VisitGetOperationEnd,
                },
                call_stack,
            )?;
            visit_operation(
                parsed_spec,
                out_path,
                &path_item.delete,
                &BracketScripts {
                    start: Script::VisitDeleteOperationStart,
                    end: Script::VisitDeleteOperationEnd,
                },
                call_stack,
            )?;
            visit_servers(
                out_path,
                &path_item.servers,
                &path_item.extensions,
                call_stack,
            )?;
            //Not so effective, but used existing visitor
            visit_parameters(
                parsed_spec,
                out_path,
                &path_item
                    .parameters
                    .iter()
                    .enumerate()
                    .map(|(index, value)| (index.to_string(), value.clone()))
                    .collect::<IndexMap<String, ReferenceOr<Parameter>>>(),
                &path_item.extensions,
                call_stack,
            )?;
            Ok(())
        })?
        .and_then(|call_stack| {
            Script::VisitPathItemEnd.call_with_descriptor(
                out_path,
                &(path_item_name, &path_item, &path_item.extensions),
                call_stack,
            )?;
            Ok(())
        })?;
    Ok(())
}

pub fn visit_operation(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    operation: &Option<Operation>,
    braced_scripts: &BracketScripts,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(operation) = operation {
        braced_scripts
            .start
            .call_with_descriptor(out_path, &(&operation, &operation.extensions), call_stack)?
            .and_then(|call_stack| {
                visit_external_docs(out_path, &operation.external_docs, call_stack)?;
                visit_parameters(
                    parsed_spec,
                    out_path,
                    &operation
                        .parameters
                        .iter()
                        .enumerate()
                        .map(|(index, value)| (index.to_string(), value.clone()))
                        .collect::<IndexMap<String, ReferenceOr<Parameter>>>(),
                    &operation.extensions,
                    call_stack,
                )?;

                if let Some(request_body) = &operation.request_body {
                    visit_request_body(parsed_spec, out_path, None, request_body, call_stack)?;
                }

                visit_operation_responses(parsed_spec, out_path, &operation.responses, call_stack)?;

                let operation_callbacks: IndexMap<String, ReferenceOr<Callback>> = operation
                    .callbacks
                    .iter()
                    .map(|(key, value)| (key.clone(), ReferenceOr::Item(value.clone())))
                    .collect();

                visit_callbacks(
                    parsed_spec,
                    out_path,
                    &operation_callbacks,
                    &operation.extensions,
                    call_stack,
                )?;
                visit_security_requirements(
                    out_path,
                    &operation.security,
                    &operation.extensions,
                    call_stack,
                )?;
                visit_servers(
                    out_path,
                    &operation.servers,
                    &operation.extensions,
                    call_stack,
                )?;
                Ok(())
            })?
            .and_then(|call_stack| {
                braced_scripts.end.call_with_descriptor(
                    out_path,
                    &(&operation, &operation.extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_cookie_parameter(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameter_name: &str,
    parameter: &Parameter,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Parameter::Cookie { parameter_data, .. } = parameter {
        Script::VisitCookieParameterStart
            .call_with_descriptor(
                out_path,
                &(parameter_name, &parameter, &extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_parameter_data(parsed_spec, out_path, parameter_data, call_stack)?;
                Ok(())
            })?
            .and_then(|call_stack| {
                Script::VisitCookieParameterEnd.call_with_descriptor(
                    out_path,
                    &(parameter_name, &parameter, &extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
        Ok(())
    } else {
        Err(anyhow!("Not a Cookie parameter"))
    }
}

pub fn visit_spec_components(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    components: &Option<Components>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(components) = components {
        Script::VisitComponentsStart
            .call_with_descriptor(out_path, &(components, &components.extensions), call_stack)?
            .and_then(|call_stack| {
                visit_schemas(
                    parsed_spec,
                    out_path,
                    &components.schemas,
                    &components.extensions,
                    call_stack,
                )?;

                visit_components_responses(
                    parsed_spec,
                    out_path,
                    &components.responses,
                    &components.extensions,
                    call_stack,
                )?;

                visit_parameters(
                    parsed_spec,
                    out_path,
                    &components.parameters,
                    &components.extensions,
                    call_stack,
                )?;

                visit_examples(
                    parsed_spec,
                    out_path,
                    &components.examples,
                    &components.extensions,
                    call_stack,
                )?;

                visit_request_bodies(
                    parsed_spec,
                    out_path,
                    &components.request_bodies,
                    &components.extensions,
                    call_stack,
                )?;

                visit_headers(
                    parsed_spec,
                    out_path,
                    &components.headers,
                    &components.extensions,
                    call_stack,
                )?;

                visit_security_schemes(
                    parsed_spec,
                    out_path,
                    &components.security_schemes,
                    &components.extensions,
                    call_stack,
                )?;

                visit_links(
                    parsed_spec,
                    out_path,
                    &components.links,
                    &components.extensions,
                    call_stack,
                )?;

                visit_callbacks(
                    parsed_spec,
                    out_path,
                    &components.callbacks,
                    &components.extensions,
                    call_stack,
                )
            })?
            .and_then(|call_stack| {
                Script::VisitComponentsEnd.call_with_descriptor(
                    out_path,
                    &(&components.extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_schema_default(
    out_path: &Path,
    default: &Option<serde_json::Value>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(default) = default.as_ref() {
        Script::VisitDefault.call_with_descriptor(out_path, &(default, extensions), call_stack)?;
    }
    Ok(())
}

pub fn visit_spec_info(out_path: &Path, info: &Info, call_stack: &CallStack) -> Result<()> {
    Script::VisitSpecInfoStart
        .call_with_descriptor(out_path, &(info, &info.extensions), call_stack)?
        .and_then(|call_stack| {
            visit_spec_info_contact(out_path, &info.contact, call_stack)?;
            visit_spec_info_license(out_path, &info.license, call_stack)
        })?
        .and_then(|call_stack| {
            Script::VisitSpecInfoEnd.call_with_descriptor(
                out_path,
                &(info, &info.extensions),
                call_stack,
            )?;
            Ok(())
        })?;
    Ok(())
}

pub fn visit_spec_info_contact(
    out_path: &Path,
    contact: &Option<Contact>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(it) = contact {
        Script::VisitSpecInfoContact.call_with_descriptor(
            out_path,
            &(&it, &it.extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_spec_info_license(
    out_path: &Path,
    license: &Option<License>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(it) = license {
        Script::VisitSpecInfoLicense.call_with_descriptor(
            out_path,
            &(&it, &it.extensions),
            call_stack,
        )?;
    }
    Ok(())
}
pub fn visit_server(out_path: &Path, server: &Server, call_stack: &CallStack) -> Result<()> {
    Script::VisitServerStart
        .call_with_descriptor(out_path, &(server, &server.extensions), call_stack)?
        .and_then(|call_stack| {
            if let Some(variables) = server.variables.as_ref() {
                variables.iter().try_for_each(|it| {
                    Script::VisitServerVariable.call_with_descriptor(
                        out_path,
                        &(&server.url, &it.0, &it.1, &it.1.extensions),
                        call_stack,
                    )?;
                    Ok(())
                })?;
            }
            Ok(())
        })?
        .and_then(|call_stack| {
            Script::VisitServerEnd.call_with_descriptor(
                out_path,
                &(server, &server.extensions),
                call_stack,
            )?;
            Ok(())
        })?;
    Ok(())
}

pub fn visit_servers(
    out_path: &Path,
    servers: &Vec<Server>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !servers.is_empty() {
        Script::VisitServersStart
            .call_with_descriptor(out_path, &(servers, extensions), call_stack)?
            .and_then(|call_stack| {
                servers.iter().try_for_each(|server| {
                    visit_server(out_path, server, call_stack)?;
                    Ok(())
                })
            })?
            .and_then(|call_stack| {
                Script::VisitServersEnd.call_with_descriptor(
                    out_path,
                    &(servers, extensions),
                    call_stack,
                )?;
                Ok(())
            })?;
    }
    Ok(())
}

pub fn visit_object_property<T>(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    property_name: &str,
    property_schema_ref: &ReferenceOr<T>,
    call_stack: &CallStack,
) -> Result<()>
where
    T: DeserializeOwned + Send + Sync + AsSchemaRef + From<Schema> + 'static,
{
    match property_schema_ref {
        ReferenceOr::Reference { reference } => {
            visit_object_property(
                parsed_spec,
                out_path,
                property_name,
                references::resolve_reference::<T>(reference, parsed_spec)?,
                call_stack,
            )?;
            Ok(())
        }
        ReferenceOr::Item(schema) => {
            let schema = schema.as_schema();

            Script::VisitObjectPropertyStart
                .call_with_descriptor(
                    out_path,
                    &(property_name, schema, &schema.schema_data.extensions),
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_schema(
                        parsed_spec,
                        out_path,
                        Some(property_name),
                        property_schema_ref,
                        call_stack,
                    )
                })?
                .and_then(|call_stack| {
                    Script::VisitObjectPropertyEnd.call_with_descriptor(
                        out_path,
                        &(property_name, schema, &schema.schema_data.extensions),
                        call_stack,
                    )?;
                    Ok(())
                })?;
            Ok(())
        }
    }
}

pub fn visit_object(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    object_description: &ObjectType,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    Script::VisitObjectStart
        .call_with_descriptor(out_path, &(object_description, extensions), call_stack)?
        .and_then(|call_stack| {
            if !object_description.properties.is_empty() {
                Script::VisitObjectPropertiesStart
                    .call_with_descriptor(
                        out_path,
                        &(&object_description.properties, extensions),
                        call_stack,
                    )?
                    .and_then(|call_stack| {
                        object_description.properties.iter().try_for_each(
                            |(local_property_name, property_schema_ref)| -> Result<()> {
                                visit_object_property(
                                    parsed_spec,
                                    out_path,
                                    local_property_name,
                                    property_schema_ref,
                                    call_stack,
                                )
                            },
                        )?;
                        Ok(())
                    })?
                    .and_then(|call_stack| {
                        Script::VisitObjectPropertiesEnd.call_with_descriptor(
                            out_path,
                            &(&object_description.properties, extensions),
                            call_stack,
                        )?;
                        Ok(())
                    })?;
            }

            if let Some(it) = object_description.additional_properties.as_ref() {
                match it {
                    openapiv3::AdditionalProperties::Any(value) => {
                        Script::VisitAdditionalPropertiesAny.call_with_descriptor(
                            out_path,
                            &(
                                *value,
                                object_description.min_properties,
                                object_description.max_properties,
                                extensions,
                            ),
                            call_stack,
                        )?;
                    }
                    openapiv3::AdditionalProperties::Schema(it) => {
                        let schema_ref = it.as_ref();
                        Script::VisitAdditionalPropertiesStart
                            .call_with_descriptor(
                                out_path,
                                &(
                                    schema_ref,
                                    object_description.min_properties,
                                    object_description.max_properties,
                                    extensions,
                                ),
                                call_stack,
                            )?
                            .and_then(|call_stack| {
                                visit_schema(parsed_spec, out_path, None, schema_ref, call_stack)
                            })?
                            .and_then(|call_stack| {
                                Script::VisitAdditionalPropertiesEnd.call_with_descriptor(
                                    out_path,
                                    &(
                                        schema_ref,
                                        object_description.min_properties,
                                        object_description.max_properties,
                                        extensions,
                                    ),
                                    call_stack,
                                )?;
                                Ok(())
                            })?;
                    }
                }
            }
            Ok(())
        })?
        .and_then(|call_stack| {
            Script::VisitObjectEnd.call_with_descriptor(
                out_path,
                &(object_description, extensions),
                call_stack,
            )?;
            Ok(())
        })?;
    Ok(())
}
