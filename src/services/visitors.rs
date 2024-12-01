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

use crate::{
    enums::common::Script,
    holders::context::CACHE,
    services::{comparators::assert_diff, references},
    structs::common::{BracketScripts, ParsedSpec},
    traits::common::AsSchemaRef,
    Commands,
};
use anyhow::{anyhow, Context};
use anyhow::{Ok, Result};

use super::{cli, scripts::get_call_id};

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

        cli::set_global_lua_parameters(&openapi)?;

        let parsed_spec = ParsedSpec {
            path: spec_path.to_owned(),
            spec: Arc::new(spec_as_json),
        };

        Script::VisitSpecStart.call_with_descriptor(
            spec_path.to_str(),
            out_path,
            &(&openapi.openapi, &openapi.extensions),
        )?;

        visit_spec_info(out_path, &openapi.info)?;
        visit_servers(out_path, &openapi.servers, &openapi.extensions)?;
        visit_paths(&parsed_spec, out_path, &openapi.paths)?;
        visit_security_requirements(out_path, &openapi.security, &openapi.extensions)?;
        visit_spec_tags(out_path, &openapi.tags, &openapi.extensions)?;
        visit_external_docs(out_path, &openapi.external_docs)?;
        visit_spec_components(&parsed_spec, out_path, &openapi.components)?;

        Script::VisitSpecEnd.call_with_descriptor(
            spec_path.to_str(),
            out_path,
            &(&openapi.openapi, &openapi.extensions),
        )?;

        if let Some(expected_path) = expected {
            assert_diff(out_path, expected_path)?;
        }
        info!("Command execution end for [{:?}]", spec_path);
        Ok(())
    } else {
        Err(anyhow!("Expected a Translate command"))
    }
}

pub fn visit_not(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    schema_ref: &ReferenceOr<Schema>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    Script::VisitPropertyNotStart.call_with_descriptor(
        None,
        out_path,
        &(schema_ref, extensions),
    )?;
    visit_schema(parsed_spec, out_path, None, schema_ref)?;
    Script::VisitPropertyNotEnd.call_with_descriptor(None, out_path, &(schema_ref, extensions))
}

pub fn visit_schema(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    schema_name: Option<&str>,
    schema_ref: &ReferenceOr<Schema>,
) -> Result<()> {
    match schema_ref {
        ReferenceOr::Reference { reference } => {
            let schema = references::resolve_reference::<Schema>(reference, parsed_spec)?;
            Script::VisitSchemaReferenceStart.call_with_descriptor(
                get_call_id(schema_name, reference).as_deref(),
                out_path,
                &(
                    schema_name,
                    reference,
                    &schema
                        .as_item()
                        .expect("Unable to get schema from resolved reference")
                        .schema_data
                        .extensions,
                ),
            )?;

            visit_schema(parsed_spec, out_path, None, schema)?;
            Script::VisitSchemaReferenceEnd.call_with_descriptor(
                get_call_id(schema_name, reference).as_deref(),
                out_path,
                &(
                    schema_name,
                    reference,
                    &schema
                        .as_item()
                        .expect("Unable to get schema from resolved reference")
                        .schema_data
                        .extensions,
                ),
            )
        }
        ReferenceOr::Item(schema_item) => {
            let schema_extensions = &schema_item.as_schema().schema_data.extensions;

            let schema_data = &schema_item.as_schema().schema_data;

            Script::VisitSchemaStart.call_with_descriptor(
                schema_name,
                out_path,
                &(schema_name, &schema_data, &schema_extensions),
            )?;

            visit_discriminator(out_path, &schema_data.discriminator)?;

            visit_external_docs(out_path, &schema_data.external_docs)?;

            visit_generic_example(out_path, &schema_data.example, &schema_data.extensions)?;

            visit_schema_default(out_path, &schema_data.default, &schema_data.extensions)?;

            match &schema_item.as_schema().schema_kind {
                openapiv3::SchemaKind::Type(type_) => match type_ {
                    openapiv3::Type::Object(object_descriptor) => {
                        visit_object(parsed_spec, out_path, object_descriptor, schema_extensions)
                    }
                    openapiv3::Type::Array(array_descriptor) => {
                        visit_array(parsed_spec, out_path, array_descriptor, schema_extensions)
                    }
                    // Simple types
                    openapiv3::Type::String(string_descriptor) => {
                        visit_string(out_path, string_descriptor, schema_extensions)
                    }
                    openapiv3::Type::Number(number_descriptor) => {
                        visit_number(out_path, number_descriptor, schema_extensions)
                    }
                    openapiv3::Type::Integer(integer_descriptor) => {
                        visit_integer(out_path, integer_descriptor, schema_extensions)
                    }
                    openapiv3::Type::Boolean(boolean_descriptor) => {
                        visit_boolean(out_path, boolean_descriptor, schema_extensions)
                    }
                },
                openapiv3::SchemaKind::OneOf { one_of } => visit_group_of(
                    parsed_spec,
                    out_path,
                    one_of,
                    &BracketScripts {
                        start: Script::VisitOneOfStart,
                        end: Script::VisitOneOfEnd,
                    },
                    &BracketScripts {
                        start: Script::VisitOneOfElementStart,
                        end: Script::VisitOneOfElementEnd,
                    },
                    schema_extensions,
                ),
                openapiv3::SchemaKind::AllOf { all_of } => visit_group_of(
                    parsed_spec,
                    out_path,
                    all_of,
                    &BracketScripts {
                        start: Script::VisitAllOfStart,
                        end: Script::VisitAllOfEnd,
                    },
                    &BracketScripts {
                        start: Script::VisitAllOfElementStart,
                        end: Script::VisitAllOfElementEnd,
                    },
                    schema_extensions,
                ),
                openapiv3::SchemaKind::AnyOf { any_of } => visit_group_of(
                    parsed_spec,
                    out_path,
                    any_of,
                    &BracketScripts {
                        start: Script::VisitAnyOfStart,
                        end: Script::VisitAnyOfEnd,
                    },
                    &BracketScripts {
                        start: Script::VisitAnyOfElementStart,
                        end: Script::VisitAnyOfElementEnd,
                    },
                    schema_extensions,
                ),
                openapiv3::SchemaKind::Not { not } => {
                    let unboxed = not.as_ref();
                    visit_not(parsed_spec, out_path, unboxed, schema_extensions)
                }
                openapiv3::SchemaKind::Any(any_schema) => visit_any_schema(
                    parsed_spec,
                    out_path,
                    schema_name,
                    any_schema,
                    schema_extensions,
                ),
            }?;

            Script::VisitSchemaEnd.call_with_descriptor(
                schema_name,
                out_path,
                &(schema_name, schema_data, schema_extensions),
            )
        }
    }
}

pub fn visit_response(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    response_name: Option<&str>,
    response_ref: &ReferenceOr<Response>,
) -> Result<()> {
    match response_ref {
        ReferenceOr::Reference { reference } => {
            let response = references::resolve_reference::<Response>(reference, parsed_spec)?;
            Script::VisitResponseReferenceStart.call_with_descriptor(
                get_call_id(response_name, reference).as_deref(),
                out_path,
                &(
                    response_name,
                    reference,
                    &response
                        .as_item()
                        .expect("Unable to get extensions from resolved response")
                        .extensions,
                ),
            )?;

            visit_response(parsed_spec, out_path, None, response)?;
            Script::VisitResponseReferenceEnd.call_with_descriptor(
                get_call_id(response_name, reference).as_deref(),
                out_path,
                &(
                    response_name,
                    reference,
                    &response
                        .as_item()
                        .expect("Unable to get extensions from resolved response")
                        .extensions,
                ),
            )
        }
        ReferenceOr::Item(response) => {
            let response_extensions = &response.extensions;

            Script::VisitResponseStart.call_with_descriptor(
                response_name,
                out_path,
                &(&response_name, response, &response_extensions),
            )?;

            visit_headers(
                parsed_spec,
                out_path,
                &response.headers,
                response_extensions,
            )?;

            visit_media_types(
                parsed_spec,
                out_path,
                &response.content,
                response_extensions,
            )?;

            visit_links(parsed_spec, out_path, &response.links, response_extensions)?;

            Script::VisitResponseEnd.call_with_descriptor(
                response_name,
                out_path,
                &(response_name, response, response_extensions),
            )
        }
    }
}

pub fn visit_string(
    out_path: &Path,
    string_descriptor: &StringType,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    Script::VisitStringProperty.call_with_descriptor(
        None,
        out_path,
        &(string_descriptor, extensions),
    )
}

pub fn visit_any_schema(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    schema_name: Option<&str>,
    any_schema_descriptor: &AnySchema,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    Script::VisitAnySchemaStart.call_with_descriptor(
        schema_name,
        out_path,
        &(any_schema_descriptor, extensions),
    )?;
    if !any_schema_descriptor.all_of.is_empty() {
        visit_group_of(
            parsed_spec,
            out_path,
            &any_schema_descriptor.all_of,
            &BracketScripts {
                start: Script::VisitAllOfStart,
                end: Script::VisitAllOfEnd,
            },
            &BracketScripts {
                start: Script::VisitAllOfElementStart,
                end: Script::VisitAllOfElementEnd,
            },
            extensions,
        )?;
    }

    if !any_schema_descriptor.any_of.is_empty() {
        visit_group_of(
            parsed_spec,
            out_path,
            &any_schema_descriptor.any_of,
            &BracketScripts {
                start: Script::VisitAnyOfStart,
                end: Script::VisitAnyOfEnd,
            },
            &BracketScripts {
                start: Script::VisitAnyOfElementStart,
                end: Script::VisitAnyOfElementEnd,
            },
            extensions,
        )?;
    }
    if !any_schema_descriptor.one_of.is_empty() {
        visit_group_of(
            parsed_spec,
            out_path,
            &any_schema_descriptor.one_of,
            &BracketScripts {
                start: Script::VisitOneOfStart,
                end: Script::VisitOneOfEnd,
            },
            &BracketScripts {
                start: Script::VisitOneOfElementStart,
                end: Script::VisitOneOfElementEnd,
            },
            extensions,
        )?;
    }

    if let Some(schema) = any_schema_descriptor.not.as_ref() {
        visit_not(parsed_spec, out_path, schema, extensions)?;
    }

    // visit_object(parsed_spec, out_path, object_description, extensions, )?;
    if let Some(schema) = any_schema_descriptor.typ.as_ref() {
        match schema.as_str() {
            "string" => visit_string(
                out_path,
                &serde_json::from_value(serde_json::to_value(any_schema_descriptor)?)?,
                extensions,
            ),
            "number" => visit_number(
                out_path,
                &serde_json::from_value(serde_json::to_value(any_schema_descriptor)?)?,
                extensions,
            ),
            "integer" => visit_integer(
                out_path,
                &serde_json::from_value(serde_json::to_value(any_schema_descriptor)?)?,
                extensions,
            ),
            "boolean" => visit_boolean(
                out_path,
                &serde_json::from_value(serde_json::to_value(any_schema_descriptor)?)?,
                extensions,
            ),
            "array" => visit_array(
                parsed_spec,
                out_path,
                &serde_json::from_value(serde_json::to_value(any_schema_descriptor)?)?,
                extensions,
            ),
            "object" => visit_object(
                parsed_spec,
                out_path,
                &serde_json::from_value(serde_json::to_value(any_schema_descriptor)?)?,
                extensions,
            ),
            _ => Ok(()),
        }?;
    }

    Script::VisitAnySchemaEnd.call_with_descriptor(
        schema_name,
        out_path,
        &(any_schema_descriptor, extensions),
    )
}

pub fn visit_number(
    out_path: &Path,
    number_descriptor: &NumberType,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    Script::VisitNumberProperty.call_with_descriptor(
        None,
        out_path,
        &(number_descriptor, extensions),
    )
}

pub fn visit_integer(
    out_path: &Path,
    integer_descriptor: &IntegerType,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    Script::VisitIntegerProperty.call_with_descriptor(
        None,
        out_path,
        &(integer_descriptor, extensions),
    )
}

pub fn visit_array(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    array_descriptor: &ArrayType,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    Script::VisitArrayPropertyStart.call_with_descriptor(
        None,
        out_path,
        &(array_descriptor, extensions),
    )?;
    if let Some(it) = &array_descriptor.items {
        let unboxed = it.clone().unbox();
        visit_schema(parsed_spec, out_path, None, &unboxed)?;
    }

    Script::VisitArrayPropertyEnd.call_with_descriptor(
        None,
        out_path,
        &(array_descriptor, extensions),
    )
}

pub fn visit_boolean(
    out_path: &Path,
    boolean_descriptor: &BooleanType,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    Script::VisitBooleanProperty.call_with_descriptor(
        None,
        out_path,
        &(boolean_descriptor, extensions),
    )
}

pub fn visit_group_of(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    schemas: &[ReferenceOr<Schema>],
    braced_scripts: &BracketScripts,
    element_scripts: &BracketScripts,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if !schemas.is_empty() {
        braced_scripts
            .start
            .call_with_descriptor(None, out_path, &(schemas, &extensions))?;
        schemas.iter().enumerate().try_for_each(|(index, schema)| {
            element_scripts.start.call_with_descriptor(
                Some(&index.to_string()),
                out_path,
                &(schema, &extensions),
            )?;
            visit_schema(parsed_spec, out_path, None, schema)?;
            element_scripts.end.call_with_descriptor(
                Some(&index.to_string()),
                out_path,
                &(schema, &extensions),
            )
        })?;

        braced_scripts
            .end
            .call_with_descriptor(None, out_path, &(schemas, extensions))
    } else {
        Ok(())
    }
}

pub fn visit_discriminator(out_path: &Path, dicriminator: &Option<Discriminator>) -> Result<()> {
    if let Some(discriminator) = dicriminator.as_ref() {
        Script::VisitDiscriminator.call_with_descriptor(
            Some(&discriminator.property_name),
            out_path,
            &(discriminator, &discriminator.extensions),
        )
    } else {
        Ok(())
    }
}

pub fn visit_generic_example(
    out_path: &Path,
    example: &Option<serde_json::Value>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if let Some(example) = example {
        Script::VisitGenericExample.call_with_descriptor(None, out_path, &(&example, extensions))
    } else {
        Ok(())
    }
}

pub fn visit_generic_parameter(
    out_path: &Path,
    parameter_name: &str,
    parameter: &serde_json::Value,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    Script::VisitGenericParameter.call_with_descriptor(
        Some(parameter_name),
        out_path,
        &(parameter_name, parameter, extensions),
    )
}

pub fn visit_generic_request_body(
    out_path: &Path,
    body: &Option<serde_json::Value>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if let Some(body_json) = body {
        Script::VisitGenericRequestBody.call_with_descriptor(
            None,
            out_path,
            &(&body_json, extensions),
        )
    } else {
        Ok(())
    }
}

pub fn visit_media_type_encodings(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    encodings: &IndexMap<String, Encoding>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if !encodings.is_empty() {
        Script::VisitEncodingsStart.call_with_descriptor(
            None,
            out_path,
            &(&encodings, extensions),
        )?;

        for encoding in encodings {
            visit_media_type_encoding(parsed_spec, out_path, encoding.0, encoding.1)?;
        }

        Script::VisitEncodingsEnd.call_with_descriptor(None, out_path, &(&encodings, extensions))
    } else {
        Ok(())
    }
}

pub fn visit_media_type_encoding(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    encoding_name: &str,
    encoding: &Encoding,
) -> Result<()> {
    Script::VisitEncodingStart.call_with_descriptor(
        Some(encoding_name),
        out_path,
        &(&encoding_name, &encoding, &encoding.extensions),
    )?;
    visit_headers(
        parsed_spec,
        out_path,
        &encoding.headers,
        &encoding.extensions,
    )?;

    Script::VisitEncodingEnd.call_with_descriptor(
        Some(encoding_name),
        out_path,
        &(&encoding_name, &encoding, &encoding.extensions),
    )
}

pub fn visit_example(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    example_name: Option<&str>,
    example_ref: &ReferenceOr<Example>,
) -> Result<()> {
    match example_ref {
        ReferenceOr::Reference { reference } => {
            let example = references::resolve_reference::<Example>(reference, parsed_spec)?;
            Script::VisitExampleReferenceStart.call_with_descriptor(
                get_call_id(example_name, reference).as_deref(),
                out_path,
                &(
                    example_name,
                    reference,
                    &example
                        .as_item()
                        .expect("Unable to get example from resolved reference")
                        .extensions,
                ),
            )?;

            visit_example(parsed_spec, out_path, None, example)?;
            Script::VisitExampleReferenceEnd.call_with_descriptor(
                get_call_id(example_name, reference).as_deref(),
                out_path,
                &(
                    example_name,
                    reference,
                    &example
                        .as_item()
                        .expect("Unable to get example from resolved reference")
                        .extensions,
                ),
            )
        }
        ReferenceOr::Item(example) => {
            Script::VisitExampleStart.call_with_descriptor(
                example_name,
                out_path,
                &(example_name, &example, &example.extensions),
            )?;

            visit_generic_example(out_path, &example.value, &example.extensions)?;

            Script::VisitExampleEnd.call_with_descriptor(
                example_name,
                out_path,
                &(example_name, &example, &example.extensions),
            )
        }
    }
}

pub fn visit_request_body(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    request_body_name: Option<&str>,
    request_body_ref: &ReferenceOr<RequestBody>,
) -> Result<()> {
    match request_body_ref {
        ReferenceOr::Reference { reference } => {
            let request_body =
                references::resolve_reference::<RequestBody>(reference, parsed_spec)?;
            Script::VisitRequestBodyReferenceStart.call_with_descriptor(
                get_call_id(request_body_name, reference).as_deref(),
                out_path,
                &(
                    &request_body_name,
                    &reference,
                    &request_body
                        .as_item()
                        .expect("Unable to get extensions from resolved request body")
                        .extensions,
                ),
            )?;

            visit_request_body(parsed_spec, out_path, None, request_body)?;
            Script::VisitRequestBodyReferenceEnd.call_with_descriptor(
                get_call_id(request_body_name, reference).as_deref(),
                out_path,
                &(
                    &request_body_name,
                    &reference,
                    &request_body
                        .as_item()
                        .expect("Unable to get extensions from resolved request body")
                        .extensions,
                ),
            )
        }
        ReferenceOr::Item(request_body) => {
            Script::VisitRequestBodyStart.call_with_descriptor(
                request_body_name,
                out_path,
                &(&request_body_name, &request_body, &request_body.extensions),
            )?;

            visit_media_types(
                parsed_spec,
                out_path,
                &request_body.content,
                &request_body.extensions,
            )?;

            Script::VisitRequestBodyEnd.call_with_descriptor(
                request_body_name,
                out_path,
                &(&request_body_name, &request_body, &request_body.extensions),
            )
        }
    }
}

pub fn visit_parameter_schema_or_content(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameter_name: Option<&str>,
    parameter_schema_or_content: &ParameterSchemaOrContent,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    Script::VisitParameterSchemaOrContentStart.call_with_descriptor(
        parameter_name,
        out_path,
        &(&parameter_name, &parameter_schema_or_content, extensions),
    )?;
    match parameter_schema_or_content {
        ParameterSchemaOrContent::Schema(schema_ref) => {
            visit_schema(parsed_spec, out_path, None, schema_ref)?;
        }
        ParameterSchemaOrContent::Content(media_types) => {
            visit_media_types(parsed_spec, out_path, media_types, extensions)?;
        }
    }

    Script::VisitParameterSchemaOrContentEnd.call_with_descriptor(
        parameter_name,
        out_path,
        &(&parameter_name, &parameter_schema_or_content, extensions),
    )
}

pub fn visit_media_types(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    media_types: &IndexMap<String, MediaType>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if !media_types.is_empty() {
        Script::VisitMediaTypesStart.call_with_descriptor(
            None,
            out_path,
            &(media_types, extensions),
        )?;

        for media_type in media_types {
            visit_media_type(parsed_spec, out_path, media_type.0, media_type.1)?;
        }

        Script::VisitMediaTypesEnd.call_with_descriptor(None, out_path, &(media_types, extensions))
    } else {
        Ok(())
    }
}

pub fn visit_callbacks(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    operation_callbacks: &IndexMap<String, ReferenceOr<Callback>>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if !operation_callbacks.is_empty() {
        Script::VisitAsyncCallbacksStart.call_with_descriptor(
            None,
            out_path,
            &(operation_callbacks, &extensions),
        )?;

        for callbacks in operation_callbacks {
            visit_callback(
                parsed_spec,
                out_path,
                Some(callbacks.0),
                callbacks.1,
                extensions,
            )?;
        }

        Script::VisitAsyncCallbacksEnd.call_with_descriptor(
            None,
            out_path,
            &(operation_callbacks, &extensions),
        )
    } else {
        Ok(())
    }
}

pub fn visit_links(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    links: &IndexMap<String, ReferenceOr<Link>>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if !links.is_empty() {
        Script::VisitLinksStart.call_with_descriptor(None, out_path, &(links, &extensions))?;

        for link in links {
            visit_link(parsed_spec, out_path, Some(link.0), link.1)?;
        }

        Script::VisitLinksEnd.call_with_descriptor(None, out_path, &(links, &extensions))
    } else {
        Ok(())
    }
}

pub fn visit_link(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    link_name: Option<&str>,
    link_ref: &ReferenceOr<Link>,
) -> Result<()> {
    match link_ref {
        ReferenceOr::Reference { reference } => {
            let link = references::resolve_reference::<Link>(reference, parsed_spec)?;
            Script::VisitLinkReferenceStart.call_with_descriptor(
                get_call_id(link_name, reference).as_deref(),
                out_path,
                &(
                    link_name,
                    reference,
                    &link
                        .as_item()
                        .expect("Unable to get extensions from resolved link reference")
                        .extensions,
                ),
            )?;
            visit_link(parsed_spec, out_path, None, link)?;
            Script::VisitLinkReferenceEnd.call_with_descriptor(
                get_call_id(link_name, reference).as_deref(),
                out_path,
                &(
                    link_name,
                    reference,
                    &link
                        .as_item()
                        .expect("Unable to get extensions from resolved link reference")
                        .extensions,
                ),
            )
        }
        ReferenceOr::Item(link) => {
            Script::VisitLinkStart.call_with_descriptor(
                link_name,
                out_path,
                &(link_name, link, &link.extensions),
            )?;

            visit_generic_request_body(out_path, &link.request_body, &link.extensions)?;

            visit_generic_parameters(out_path, &link.parameters, &link.extensions)?;

            if let Some(server) = &link.server {
                visit_server(out_path, server)?;
            }

            Script::VisitLinkEnd.call_with_descriptor(
                link_name,
                out_path,
                &(link_name, link, &link.extensions),
            )
        }
    }
}

pub fn visit_callback(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    callbacks_name: Option<&str>,
    callbacks: &ReferenceOr<Callback>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    match callbacks {
        ReferenceOr::Reference { reference } => {
            let callback = references::resolve_reference::<Callback>(reference, parsed_spec)?;
            Script::VisitAsyncCallbackReferenceStart.call_with_descriptor(
                get_call_id(callbacks_name, reference).as_deref(),
                out_path,
                &(callbacks_name, reference, &extensions),
            )?;

            visit_callback(parsed_spec, out_path, None, callback, extensions)?;
            Script::VisitAsyncCallbackReferenceEnd.call_with_descriptor(
                get_call_id(callbacks_name, reference).as_deref(),
                out_path,
                &(callbacks_name, reference, &extensions),
            )
        }
        ReferenceOr::Item(callback) => {
            Script::VisitAsyncCallbackStart.call_with_descriptor(
                callbacks_name,
                out_path,
                &(callbacks_name, callback, &extensions),
            )?;

            for it in callback {
                visit_path_item(parsed_spec, out_path, Some(it.0), it.1)?;
            }

            Script::VisitAsyncCallbackEnd.call_with_descriptor(
                callbacks_name,
                out_path,
                &(callbacks_name, callback, &extensions),
            )
        }
    }
}

pub fn visit_media_type(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    media_type_name: &str,
    media_type: &MediaType,
) -> Result<()> {
    Script::VisitMediaTypeStart.call_with_descriptor(
        Some(media_type_name),
        out_path,
        &(&media_type_name, media_type, &media_type.extensions),
    )?;
    if let Some(schema_ref) = &media_type.schema {
        visit_schema(parsed_spec, out_path, None, schema_ref)?;
    }

    visit_generic_example(out_path, &media_type.example, &media_type.extensions)?;

    visit_examples(
        parsed_spec,
        out_path,
        &media_type.examples,
        &media_type.extensions,
    )?;

    visit_media_type_encodings(
        parsed_spec,
        out_path,
        &media_type.encoding,
        &media_type.extensions,
    )?;

    Script::VisitMediaTypeEnd.call_with_descriptor(
        Some(media_type_name),
        out_path,
        &(&media_type_name, media_type, &media_type.extensions),
    )
}

pub fn visit_examples(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    examples: &IndexMap<String, ReferenceOr<Example>>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if !examples.is_empty() {
        Script::VisitExamplesStart.call_with_descriptor(
            None,
            out_path,
            &(&examples, extensions),
        )?;

        for it in examples {
            visit_example(parsed_spec, out_path, Some(it.0), it.1)?;
        }

        Script::VisitExamplesEnd.call_with_descriptor(None, out_path, &(&examples, extensions))
    } else {
        Ok(())
    }
}

pub fn visit_request_bodies(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    request_bodies: &IndexMap<String, ReferenceOr<RequestBody>>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if !request_bodies.is_empty() {
        Script::VisitRequestBodiesStart.call_with_descriptor(
            None,
            out_path,
            &(request_bodies, extensions),
        )?;

        for it in request_bodies {
            visit_request_body(parsed_spec, out_path, Some(it.0), it.1)?;
        }

        Script::VisitRequestBodiesEnd.call_with_descriptor(
            None,
            out_path,
            &(request_bodies, extensions),
        )
    } else {
        Ok(())
    }
}

pub fn visit_generic_parameters(
    out_path: &Path,
    parameters: &IndexMap<String, serde_json::Value>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if !parameters.is_empty() {
        Script::VisitGenericParametersStart.call_with_descriptor(
            None,
            out_path,
            &(&parameters, extensions),
        )?;

        for it in parameters {
            visit_generic_parameter(out_path, it.0, it.1, extensions)?;
        }

        Script::VisitGenericParametersEnd.call_with_descriptor(
            None,
            out_path,
            &(&parameters, extensions),
        )
    } else {
        Ok(())
    }
}

pub fn visit_header(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    header_name: Option<&str>,
    header: &ReferenceOr<Header>,
) -> Result<()> {
    match header {
        ReferenceOr::Reference { reference } => {
            let header = references::resolve_reference::<Header>(reference, parsed_spec)?;
            Script::VisitHeaderReferenceStart.call_with_descriptor(
                get_call_id(header_name, reference).as_deref(),
                out_path,
                &(
                    &header_name,
                    reference,
                    &header
                        .as_item()
                        .expect("Unable to get extensions from resolved header reference")
                        .extensions,
                ),
            )?;

            visit_header(parsed_spec, out_path, None, header)?;
            Script::VisitHeaderReferenceEnd.call_with_descriptor(
                get_call_id(header_name, reference).as_deref(),
                out_path,
                &(
                    &header_name,
                    reference,
                    &header
                        .as_item()
                        .expect("Unable to get extensions from resolved header reference")
                        .extensions,
                ),
            )
        }
        ReferenceOr::Item(header) => {
            Script::VisitHeaderStart.call_with_descriptor(
                header_name,
                out_path,
                &(&header_name, &header, &header.extensions),
            )?;

            visit_parameter_schema_or_content(
                parsed_spec,
                out_path,
                None,
                &header.format,
                &header.extensions,
            )?;

            visit_generic_example(out_path, &header.example, &header.extensions)?;

            visit_examples(parsed_spec, out_path, &header.examples, &header.extensions)?;

            Script::VisitHeaderEnd.call_with_descriptor(
                header_name,
                out_path,
                &(&header_name, &header, &header.extensions),
            )
        }
    }
}

pub fn visit_security_scheme(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    scheme_name: Option<&str>,
    security_scheme: &ReferenceOr<SecurityScheme>,
) -> Result<()> {
    match security_scheme {
        ReferenceOr::Reference { reference } => {
            let scheme = references::resolve_reference::<SecurityScheme>(reference, parsed_spec)?;
            Script::VisitSecuritySchemeReferenceStart.call_with_descriptor(
                get_call_id(scheme_name, reference).as_deref(),
                out_path,
                &(
                    &scheme_name,
                    reference,
                    match &scheme
                        .as_item()
                        .expect("Unable to get extensions from resolved security scheme reference")
                    {
                        SecurityScheme::APIKey { extensions, .. } => extensions,
                        SecurityScheme::HTTP { extensions, .. } => extensions,
                        SecurityScheme::OAuth2 { extensions, .. } => extensions,
                        SecurityScheme::OpenIDConnect { extensions, .. } => extensions,
                    },
                ),
            )?;

            visit_security_scheme(parsed_spec, out_path, None, scheme)?;

            Script::VisitSecuritySchemeReferenceEnd.call_with_descriptor(
                get_call_id(scheme_name, reference).as_deref(),
                out_path,
                &(
                    &scheme_name,
                    reference,
                    match &scheme
                        .as_item()
                        .expect("Unable to get extensions from resolved security scheme reference")
                    {
                        SecurityScheme::APIKey { extensions, .. } => extensions,
                        SecurityScheme::HTTP { extensions, .. } => extensions,
                        SecurityScheme::OAuth2 { extensions, .. } => extensions,
                        SecurityScheme::OpenIDConnect { extensions, .. } => extensions,
                    },
                ),
            )
        }
        ReferenceOr::Item(security_scheme) => match security_scheme {
            SecurityScheme::APIKey { .. } => {
                visit_security_scheme_apikey(out_path, scheme_name, security_scheme)
            }

            SecurityScheme::HTTP { .. } => {
                visit_security_scheme_http(out_path, scheme_name, security_scheme)
            }

            SecurityScheme::OAuth2 { .. } => {
                visit_security_scheme_oauth2(out_path, scheme_name, security_scheme)
            }

            SecurityScheme::OpenIDConnect { .. } => {
                visit_security_scheme_openid_connect(out_path, scheme_name, security_scheme)
            }
        },
    }
}

pub fn visit_headers(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    headers: &IndexMap<String, ReferenceOr<Header>>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if !headers.is_empty() {
        Script::VisitHeadersStart.call_with_descriptor(None, out_path, &(headers, extensions))?;

        for it in headers {
            visit_header(parsed_spec, out_path, Some(it.0), it.1)?;
        }

        Script::VisitHeadersEnd.call_with_descriptor(None, out_path, &(headers, extensions))
    } else {
        Ok(())
    }
}

pub fn visit_security_schemes(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    security_schemes: &IndexMap<String, ReferenceOr<SecurityScheme>>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if !security_schemes.is_empty() {
        Script::VisitSecuritySchemesStart.call_with_descriptor(
            None,
            out_path,
            &(&security_schemes, extensions),
        )?;

        for it in security_schemes {
            visit_security_scheme(parsed_spec, out_path, Some(it.0), it.1)?;
        }

        Script::VisitSecuritySchemesEnd.call_with_descriptor(
            None,
            out_path,
            &(&security_schemes, extensions),
        )
    } else {
        Ok(())
    }
}

pub fn visit_spec_tags(
    out_path: &Path,
    tags: &Vec<Tag>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if !tags.is_empty() {
        Script::VisitSpecTagsStart.call_with_descriptor(None, out_path, &(tags, extensions))?;

        for tag in tags {
            visit_external_docs(out_path, &tag.external_docs)?;
            Script::VisitSpecTag.call_with_descriptor(
                Some(&tag.name),
                out_path,
                &(tag, &tag.extensions),
            )?;
        }

        Script::VisitSpecTagsEnd.call_with_descriptor(None, out_path, &(tags, extensions))
    } else {
        Ok(())
    }
}

pub fn visit_security_requirements(
    out_path: &Path,
    securities: &Option<Vec<SecurityRequirement>>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if let Some(it) = securities.as_ref() {
        if !it.is_empty() {
            Script::VisitSecurityRequirementsStart.call_with_descriptor(
                None,
                out_path,
                &(it, extensions),
            )?;

            for sec_map in it {
                Script::VisitSecurityRequirement.call_with_descriptor(
                    None,
                    out_path,
                    &(sec_map, extensions),
                )?;
            }

            Script::VisitSecurityRequirementsEnd.call_with_descriptor(
                None,
                out_path,
                &(it, extensions),
            )
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}
pub fn visit_external_docs(
    out_path: &Path,
    external_docs: &Option<ExternalDocumentation>,
) -> Result<()> {
    if let Some(it) = external_docs {
        Script::VisitExternalDocs.call_with_descriptor(
            Some(&it.url),
            out_path,
            &(it, &it.extensions),
        )
    } else {
        Ok(())
    }
}
pub fn visit_schemas(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    schemas: &IndexMap<String, ReferenceOr<Schema>>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if !schemas.is_empty() {
        Script::VisitSchemasStart.call_with_descriptor(None, out_path, &(schemas, extensions))?;

        for (schema_name, schema_ref) in schemas {
            visit_schema(parsed_spec, out_path, Some(schema_name), schema_ref)?;
        }

        Script::VisitSchemasEnd.call_with_descriptor(None, out_path, &(schemas, extensions))
    } else {
        Ok(())
    }
}

pub fn visit_responses(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    responses: &IndexMap<String, ReferenceOr<Response>>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if !responses.is_empty() {
        Script::VisitResponsesStart.call_with_descriptor(
            None,
            out_path,
            &(responses, extensions),
        )?;

        for (response_name, response_ref) in responses {
            visit_response(parsed_spec, out_path, Some(response_name), response_ref)?;
        }

        Script::VisitResponsesEnd.call_with_descriptor(None, out_path, &(responses, extensions))
    } else {
        Ok(())
    }
}

pub fn visit_operation_responses(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    responses: &Responses,
) -> Result<()> {
    Script::VisitOperationResponsesStart.call_with_descriptor(
        None,
        out_path,
        &(&responses, &responses.extensions),
    )?;
    if let Some(response) = &responses.default {
        visit_response(parsed_spec, out_path, None, response)?;
    }

    let converted: IndexMap<String, ReferenceOr<Response>> = responses
        .responses
        .iter()
        .map(|(key, value)| (key.to_string(), value.clone()))
        .collect();

    visit_responses(parsed_spec, out_path, &converted, &responses.extensions)?;

    Script::VisitOperationResponsesEnd.call_with_descriptor(
        None,
        out_path,
        &(&responses, &responses.extensions),
    )
}

pub fn visit_parameters(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameters: &IndexMap<String, ReferenceOr<Parameter>>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if !parameters.is_empty() {
        Script::VisitParametersStart.call_with_descriptor(
            None,
            out_path,
            &(parameters, &extensions),
        )?;

        for (parameter_name, parameter_ref) in parameters {
            visit_parameter(
                parsed_spec,
                out_path,
                Some(parameter_name),
                parameter_ref,
                extensions,
            )?;
        }

        Script::VisitParametersEnd.call_with_descriptor(None, out_path, &(parameters, &extensions))
    } else {
        Ok(())
    }
}

pub fn visit_paths(parsed_spec: &ParsedSpec, out_path: &Path, paths: &Paths) -> Result<()> {
    if !paths.paths.is_empty() {
        Script::VisitPathsStart.call_with_descriptor(
            None,
            out_path,
            &(&paths, &paths.extensions),
        )?;

        for it in &paths.paths {
            visit_path_item_ref(parsed_spec, out_path, Some(it.0), it.1)?;
        }

        Script::VisitPathsEnd.call_with_descriptor(None, out_path, &(&paths, &paths.extensions))
    } else {
        Ok(())
    }
}

pub fn visit_path_item_ref(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    path_item_name: Option<&str>,
    path_item_ref: &ReferenceOr<PathItem>,
) -> Result<()> {
    match path_item_ref {
        ReferenceOr::Reference { reference } => {
            let path_item = references::resolve_reference::<PathItem>(reference, parsed_spec)?;
            Script::VisitPathItemReferenceStart.call_with_descriptor(
                get_call_id(path_item_name, reference).as_deref(),
                out_path,
                &(
                    &path_item_name,
                    reference,
                    &path_item
                        .as_item()
                        .expect("Unable to get extensions from resolved path item reference")
                        .extensions,
                ),
            )?;

            visit_path_item_ref(parsed_spec, out_path, None, path_item)?;

            Script::VisitPathItemReferenceEnd.call_with_descriptor(
                get_call_id(path_item_name, reference).as_deref(),
                out_path,
                &(
                    &path_item_name,
                    reference,
                    &path_item
                        .as_item()
                        .expect("Unable to get extensions from resolved path item reference")
                        .extensions,
                ),
            )
        }
        ReferenceOr::Item(path_item) => {
            visit_path_item(parsed_spec, out_path, path_item_name, path_item)
        }
    }
}

//TODO: check, may be need switch from external extensions to Parameter.parameter_data.extensions
pub fn visit_parameter(
    parsed_spec: &ParsedSpec, /*  */
    out_path: &Path,
    parameter_name: Option<&str>,
    parameter_ref: &ReferenceOr<Parameter>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    match parameter_ref {
        ReferenceOr::Reference { reference } => {
            let parameter = references::resolve_reference::<Parameter>(reference, parsed_spec)?;
            Script::VisitParameterReferenceStart.call_with_descriptor(
                get_call_id(parameter_name, reference).as_deref(),
                out_path,
                &(&parameter_name, reference, extensions),
            )?;

            visit_parameter(parsed_spec, out_path, None, parameter, extensions)?;

            Script::VisitParameterReferenceEnd.call_with_descriptor(
                get_call_id(parameter_name, reference).as_deref(),
                out_path,
                &(&parameter_name, reference, extensions),
            )
        }
        ReferenceOr::Item(parameter) => match parameter {
            Parameter::Query { .. } => {
                visit_query_parameter(parsed_spec, out_path, parameter_name, parameter, extensions)
            }
            Parameter::Header { .. } => {
                visit_header_parameter(parsed_spec, out_path, parameter_name, parameter, extensions)
            }
            Parameter::Path { .. } => {
                visit_path_parameter(parsed_spec, out_path, parameter_name, parameter, extensions)
            }
            Parameter::Cookie { .. } => {
                visit_cookie_parameter(parsed_spec, out_path, parameter_name, parameter, extensions)
            }
        },
    }
}

pub fn visit_security_scheme_http(
    out_path: &Path,
    scheme_name: Option<&str>,
    http: &SecurityScheme,
) -> Result<()> {
    if let SecurityScheme::HTTP { extensions, .. } = http {
        Script::VisitSecuritySchemeHttp.call_with_descriptor(
            scheme_name,
            out_path,
            &(&scheme_name, &http, &extensions),
        )
    } else {
        Err(anyhow!("Not a SecurityScheme.HTTP"))
    }
}

pub fn visit_security_scheme_oauth2(
    out_path: &Path,
    scheme_name: Option<&str>,
    oauth2: &SecurityScheme,
) -> Result<()> {
    if let SecurityScheme::OAuth2 {
        flows, extensions, ..
    } = oauth2
    {
        Script::VisitSecuritySchemeOAuth2Start.call_with_descriptor(
            scheme_name,
            out_path,
            &(&scheme_name, &oauth2, extensions),
        )?;
        visit_security_scheme_oauth2_flows(out_path, flows)?;

        Script::VisitSecuritySchemeOAuth2End.call_with_descriptor(
            scheme_name,
            out_path,
            &(&scheme_name, &oauth2, extensions),
        )
    } else {
        Err(anyhow!("Not a SecurityScheme.OAuth2"))
    }
}

pub fn visit_security_scheme_oauth2_flows(out_path: &Path, flows: &OAuth2Flows) -> Result<()> {
    Script::VisitSecuritySchemeOAuth2FlowsStart.call_with_descriptor(
        None,
        out_path,
        &(flows, &flows.extensions),
    )?;
    visit_security_scheme_oauth2_flows_implicit(out_path, &flows.implicit)?;

    visit_security_scheme_oauth2_flows_password(out_path, &flows.password)?;

    visit_security_scheme_oauth2_flows_client_credentials(out_path, &flows.client_credentials)?;

    visit_security_scheme_oauth2_flows_authorization_code(out_path, &flows.authorization_code)?;

    Script::VisitSecuritySchemeOAuth2FlowsEnd.call_with_descriptor(
        None,
        out_path,
        &(flows, &flows.extensions),
    )
}

pub fn visit_security_scheme_oauth2_flows_implicit(
    out_path: &Path,
    flow: &Option<ImplicitOAuth2Flow>,
) -> Result<()> {
    if let Some(flow) = flow {
        Script::VisitSecuritySchemeOAuth2FlowImplicit.call_with_descriptor(
            None,
            out_path,
            &(flow, &flow.extensions),
        )
    } else {
        Ok(())
    }
}

pub fn visit_security_scheme_oauth2_flows_password(
    out_path: &Path,
    flow: &Option<PasswordOAuth2Flow>,
) -> Result<()> {
    if let Some(flow) = flow {
        Script::VisitSecuritySchemeOAuth2FlowPassword.call_with_descriptor(
            None,
            out_path,
            &(flow, &flow.extensions),
        )
    } else {
        Ok(())
    }
}

pub fn visit_security_scheme_oauth2_flows_client_credentials(
    out_path: &Path,
    flow: &Option<ClientCredentialsOAuth2Flow>,
) -> Result<()> {
    if let Some(flow) = flow {
        Script::VisitSecuritySchemeOAuth2FlowClientCredentials.call_with_descriptor(
            None,
            out_path,
            &(flow, &flow.extensions),
        )
    } else {
        Ok(())
    }
}

pub fn visit_security_scheme_oauth2_flows_authorization_code(
    out_path: &Path,
    flow: &Option<AuthorizationCodeOAuth2Flow>,
) -> Result<()> {
    if let Some(flow) = flow {
        Script::VisitSecuritySchemeOAuth2FlowAuthorizationCode.call_with_descriptor(
            None,
            out_path,
            &(flow, &flow.extensions),
        )
    } else {
        Ok(())
    }
}

pub fn visit_security_scheme_apikey(
    out_path: &Path,
    scheme_name: Option<&str>,
    api_key: &SecurityScheme,
) -> Result<()> {
    if let SecurityScheme::APIKey { extensions, .. } = api_key {
        Script::VisitSecuritySchemeApiKey.call_with_descriptor(
            scheme_name,
            out_path,
            &(scheme_name, &api_key, &extensions),
        )
    } else {
        Err(anyhow!("Not a SecurityScheme.APIKey"))
    }
}

pub fn visit_security_scheme_openid_connect(
    out_path: &Path,
    scheme_name: Option<&str>,
    openid_connect: &SecurityScheme,
) -> Result<()> {
    if let SecurityScheme::OpenIDConnect { extensions, .. } = openid_connect {
        Script::VisitSecuritySchemeOpenIdConnect.call_with_descriptor(
            scheme_name,
            out_path,
            &(scheme_name, &openid_connect, &extensions),
        )
    } else {
        Err(anyhow!("Not a SecurityScheme.OpenIDConnect"))
    }
}

pub fn visit_parameter_data(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameter_data: &ParameterData,
) -> Result<()> {
    Script::VisitParameterDataStart.call_with_descriptor(
        Some(&parameter_data.name),
        out_path,
        &(parameter_data, &parameter_data.extensions),
    )?;
    visit_parameter_schema_or_content(
        parsed_spec,
        out_path,
        None,
        &parameter_data.format,
        &parameter_data.extensions,
    )?;

    visit_generic_example(
        out_path,
        &parameter_data.example,
        &parameter_data.extensions,
    )?;

    visit_examples(
        parsed_spec,
        out_path,
        &parameter_data.examples,
        &parameter_data.extensions,
    )?;

    Script::VisitParameterDataEnd.call_with_descriptor(
        Some(&parameter_data.name),
        out_path,
        &(parameter_data, &parameter_data.extensions),
    )
}

pub fn visit_query_parameter(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameter_name: Option<&str>,
    parameter: &Parameter,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if let Parameter::Query { parameter_data, .. } = parameter {
        Script::VisitQueryParameterStart.call_with_descriptor(
            parameter_name,
            out_path,
            &(parameter_name, &parameter, &extensions),
        )?;

        visit_parameter_data(parsed_spec, out_path, parameter_data)?;

        Script::VisitQueryParameterEnd.call_with_descriptor(
            parameter_name,
            out_path,
            &(parameter_name, &parameter, &extensions),
        )
    } else {
        Err(anyhow!("Not a Query parameter"))
    }
}

pub fn visit_header_parameter(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameter_name: Option<&str>,
    parameter: &Parameter,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if let Parameter::Header { parameter_data, .. } = parameter {
        Script::VisitHeaderParameterStart.call_with_descriptor(
            parameter_name,
            out_path,
            &(parameter_name, &parameter, &extensions),
        )?;

        visit_parameter_data(parsed_spec, out_path, parameter_data)?;

        Script::VisitHeaderParameterEnd.call_with_descriptor(
            parameter_name,
            out_path,
            &(parameter_name, &parameter, &extensions),
        )
    } else {
        Err(anyhow!("Not a Header parameter"))
    }
}

pub fn visit_path_parameter(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameter_name: Option<&str>,
    parameter: &Parameter,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if let Parameter::Path { parameter_data, .. } = parameter {
        Script::VisitPathParameterStart.call_with_descriptor(
            parameter_name,
            out_path,
            &(parameter_name, &parameter, &extensions),
        )?;

        visit_parameter_data(parsed_spec, out_path, parameter_data)?;

        Script::VisitPathParameterEnd.call_with_descriptor(
            parameter_name,
            out_path,
            &(parameter_name, &parameter, &extensions),
        )
    } else {
        Err(anyhow!("Not a Path parameter"))
    }
}

pub fn visit_path_item(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    path_item_name: Option<&str>,
    path_item: &PathItem,
) -> Result<()> {
    Script::VisitPathItemStart.call_with_descriptor(
        path_item_name,
        out_path,
        &(path_item_name, &path_item, &path_item.extensions),
    )?;
    visit_operation(
        parsed_spec,
        out_path,
        &path_item.trace,
        &BracketScripts {
            start: Script::VisitTraceOperationStart,
            end: Script::VisitTraceOperationEnd,
        },
    )?;

    visit_operation(
        parsed_spec,
        out_path,
        &path_item.put,
        &BracketScripts {
            start: Script::VisitPutOperationStart,
            end: Script::VisitPutOperationEnd,
        },
    )?;

    visit_operation(
        parsed_spec,
        out_path,
        &path_item.post,
        &BracketScripts {
            start: Script::VisitPostOperationStart,
            end: Script::VisitPostOperationEnd,
        },
    )?;

    visit_operation(
        parsed_spec,
        out_path,
        &path_item.patch,
        &BracketScripts {
            start: Script::VisitPatchOperationStart,
            end: Script::VisitPatchOperationEnd,
        },
    )?;

    visit_operation(
        parsed_spec,
        out_path,
        &path_item.options,
        &BracketScripts {
            start: Script::VisitOptionsOperationStart,
            end: Script::VisitOptionsOperationEnd,
        },
    )?;

    visit_operation(
        parsed_spec,
        out_path,
        &path_item.head,
        &BracketScripts {
            start: Script::VisitHeadOperationStart,
            end: Script::VisitHeadOperationEnd,
        },
    )?;

    visit_operation(
        parsed_spec,
        out_path,
        &path_item.get,
        &BracketScripts {
            start: Script::VisitGetOperationStart,
            end: Script::VisitGetOperationEnd,
        },
    )?;

    visit_operation(
        parsed_spec,
        out_path,
        &path_item.delete,
        &BracketScripts {
            start: Script::VisitDeleteOperationStart,
            end: Script::VisitDeleteOperationEnd,
        },
    )?;

    visit_servers(out_path, &path_item.servers, &path_item.extensions)?;

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
    )?;

    Script::VisitPathItemEnd.call_with_descriptor(
        path_item_name,
        out_path,
        &(path_item_name, &path_item, &path_item.extensions),
    )
}

pub fn visit_operation(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    operation: &Option<Operation>,
    braced_scripts: &BracketScripts,
) -> Result<()> {
    if let Some(operation) = operation {
        braced_scripts.start.call_with_descriptor(
            operation
                .operation_id
                .clone()
                .or_else(|| operation.summary.clone())
                .as_deref(),
            out_path,
            &(&operation, &operation.extensions),
        )?;

        visit_external_docs(out_path, &operation.external_docs)?;

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
        )?;

        if let Some(request_body) = &operation.request_body {
            visit_request_body(parsed_spec, out_path, None, request_body)?;
        }

        visit_operation_responses(parsed_spec, out_path, &operation.responses)?;

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
        )?;

        visit_security_requirements(out_path, &operation.security, &operation.extensions)?;

        visit_servers(out_path, &operation.servers, &operation.extensions)?;

        braced_scripts.end.call_with_descriptor(
            operation
                .operation_id
                .clone()
                .or_else(|| operation.summary.clone())
                .as_deref(),
            out_path,
            &(&operation, &operation.extensions),
        )
    } else {
        Ok(())
    }
}

pub fn visit_cookie_parameter(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameter_name: Option<&str>,
    parameter: &Parameter,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if let Parameter::Cookie { parameter_data, .. } = parameter {
        Script::VisitCookieParameterStart.call_with_descriptor(
            parameter_name,
            out_path,
            &(parameter_name, &parameter, &extensions),
        )?;

        visit_parameter_data(parsed_spec, out_path, parameter_data)?;

        Script::VisitCookieParameterEnd.call_with_descriptor(
            parameter_name,
            out_path,
            &(parameter_name, &parameter, &extensions),
        )
    } else {
        Err(anyhow!("Not a Cookie parameter"))
    }
}

pub fn visit_spec_components(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    components: &Option<Components>,
) -> Result<()> {
    if let Some(components) = components {
        Script::VisitComponentsStart.call_with_descriptor(
            None,
            out_path,
            &(components, &components.extensions),
        )?;

        visit_schemas(
            parsed_spec,
            out_path,
            &components.schemas,
            &components.extensions,
        )?;

        visit_responses(
            parsed_spec,
            out_path,
            &components.responses,
            &components.extensions,
        )?;

        visit_parameters(
            parsed_spec,
            out_path,
            &components.parameters,
            &components.extensions,
        )?;

        visit_examples(
            parsed_spec,
            out_path,
            &components.examples,
            &components.extensions,
        )?;

        visit_request_bodies(
            parsed_spec,
            out_path,
            &components.request_bodies,
            &components.extensions,
        )?;

        visit_headers(
            parsed_spec,
            out_path,
            &components.headers,
            &components.extensions,
        )?;

        visit_security_schemes(
            parsed_spec,
            out_path,
            &components.security_schemes,
            &components.extensions,
        )?;

        visit_links(
            parsed_spec,
            out_path,
            &components.links,
            &components.extensions,
        )?;

        visit_callbacks(
            parsed_spec,
            out_path,
            &components.callbacks,
            &components.extensions,
        )?;

        Script::VisitComponentsEnd.call_with_descriptor(None, out_path, &(&components.extensions))
    } else {
        Ok(())
    }
}

pub fn visit_schema_default(
    out_path: &Path,
    default: &Option<serde_json::Value>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if let Some(default) = default.as_ref() {
        Script::VisitDefault.call_with_descriptor(None, out_path, &(default, extensions))
    } else {
        Ok(())
    }
}

pub fn visit_spec_info(out_path: &Path, info: &Info) -> Result<()> {
    Script::VisitSpecInfoStart.call_with_descriptor(None, out_path, &(info, &info.extensions))?;
    visit_spec_info_contact(out_path, &info.contact)?;
    visit_spec_info_license(out_path, &info.license)?;
    Script::VisitSpecInfoEnd.call_with_descriptor(None, out_path, &(info, &info.extensions))
}

pub fn visit_spec_info_contact(out_path: &Path, contact: &Option<Contact>) -> Result<()> {
    if let Some(it) = contact {
        Script::VisitSpecInfoContact.call_with_descriptor(
            it.name.as_deref(),
            out_path,
            &(&it, &it.extensions),
        )
    } else {
        Ok(())
    }
}

pub fn visit_spec_info_license(out_path: &Path, license: &Option<License>) -> Result<()> {
    if let Some(it) = license {
        Script::VisitSpecInfoLicense.call_with_descriptor(
            Some(&it.name),
            out_path,
            &(&it, &it.extensions),
        )
    } else {
        Ok(())
    }
}
pub fn visit_server(out_path: &Path, server: &Server) -> Result<()> {
    Script::VisitServerStart.call_with_descriptor(
        Some(&server.url),
        out_path,
        &(server, &server.extensions),
    )?;
    if let Some(variables) = server.variables.as_ref() {
        for it in variables {
            Script::VisitServerVariable.call_with_descriptor(
                Some(it.0),
                out_path,
                &(&server.url, &it.0, &it.1, &it.1.extensions),
            )?;
        }
    }

    Script::VisitServerEnd.call_with_descriptor(
        Some(&server.url),
        out_path,
        &(server, &server.extensions),
    )
}

pub fn visit_servers(
    out_path: &Path,
    servers: &Vec<Server>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if !servers.is_empty() {
        Script::VisitServersStart.call_with_descriptor(None, out_path, &(servers, extensions))?;

        for server in servers {
            visit_server(out_path, server)?;
        }

        Script::VisitServersEnd.call_with_descriptor(None, out_path, &(servers, extensions))
    } else {
        Ok(())
    }
}

pub fn visit_object_property(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    property_name: Option<&str>,
    property_schema_ref: &ReferenceOr<Schema>,
) -> Result<()> {
    match property_schema_ref {
        ReferenceOr::Reference { reference } => {
            let property_schema = references::resolve_reference::<Schema>(reference, parsed_spec)?;
            Script::VisitObjectPropertyReferenceStart.call_with_descriptor(
                get_call_id(property_name, reference).as_deref(),
                out_path,
                &(
                    property_name,
                    reference,
                    &property_schema
                        .as_item()
                        .expect("Unable to get extensions from resolved property schema")
                        .schema_data
                        .extensions,
                ),
            )?;

            visit_object_property(parsed_spec, out_path, None, property_schema)?;
            Script::VisitObjectPropertyReferenceEnd.call_with_descriptor(
                get_call_id(property_name, reference).as_deref(),
                out_path,
                &(
                    property_name,
                    reference,
                    &property_schema
                        .as_item()
                        .expect("Unable to get extensions from resolved property schema")
                        .schema_data
                        .extensions,
                ),
            )
        }
        ReferenceOr::Item(schema) => {
            let schema = schema.as_schema();

            Script::VisitObjectPropertyStart.call_with_descriptor(
                property_name,
                out_path,
                &(property_name, schema, &schema.schema_data.extensions),
            )?;

            visit_schema(parsed_spec, out_path, None, property_schema_ref)?;

            Script::VisitObjectPropertyEnd.call_with_descriptor(
                property_name,
                out_path,
                &(property_name, schema, &schema.schema_data.extensions),
            )
        }
    }
}

pub fn visit_object(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    object_description: &ObjectType,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    Script::VisitObjectStart.call_with_descriptor(
        None,
        out_path,
        &(object_description, extensions),
    )?;
    if !object_description.properties.is_empty() {
        Script::VisitObjectPropertiesStart.call_with_descriptor(
            None,
            out_path,
            &(&object_description.properties, extensions),
        )?;

        for (local_property_name, property_schema_ref) in &object_description.properties {
            let unboxed = property_schema_ref.clone().unbox();
            visit_object_property(parsed_spec, out_path, Some(local_property_name), &unboxed)?;
        }

        Script::VisitObjectPropertiesEnd.call_with_descriptor(
            None,
            out_path,
            &(&object_description.properties, extensions),
        )?;
    }

    if let Some(it) = object_description.additional_properties.as_ref() {
        match it {
            openapiv3::AdditionalProperties::Any(value) => {
                Script::VisitAdditionalPropertiesAny.call_with_descriptor(
                    None,
                    out_path,
                    &(
                        *value,
                        object_description.min_properties,
                        object_description.max_properties,
                        extensions,
                    ),
                )?;
            }
            openapiv3::AdditionalProperties::Schema(it) => {
                let schema_ref = it.as_ref();
                Script::VisitAdditionalPropertiesStart.call_with_descriptor(
                    None,
                    out_path,
                    &(
                        schema_ref,
                        object_description.min_properties,
                        object_description.max_properties,
                        extensions,
                    ),
                )?;

                visit_schema(parsed_spec, out_path, None, schema_ref)?;

                Script::VisitAdditionalPropertiesEnd.call_with_descriptor(
                    None,
                    out_path,
                    &(
                        schema_ref,
                        object_description.min_properties,
                        object_description.max_properties,
                        extensions,
                    ),
                )?;
            }
        }
    }

    Script::VisitObjectEnd.call_with_descriptor(None, out_path, &(object_description, extensions))
}
