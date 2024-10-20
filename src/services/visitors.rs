use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    sync::Arc,
};

use diffy::create_patch;
use indexmap::IndexMap;
use log::{error, info};
use openapiv3::{
    AnySchema, ArrayType, AuthorizationCodeOAuth2Flow, BooleanType, ClientCredentialsOAuth2Flow,
    Components, Contact, Discriminator, Encoding, Example, ExternalDocumentation, Header,
    ImplicitOAuth2Flow, Info, IntegerType, License, Link, MediaType, NumberType, OAuth2Flows,
    ObjectType, OpenAPI, Parameter, ParameterData, ParameterSchemaOrContent, PasswordOAuth2Flow,
    ReferenceOr, RequestBody, Response, Schema, SecurityRequirement, SecurityScheme, Server,
    StringType, Tag,
};
use serde::de::DeserializeOwned;

use crate::{
    enums::common::Script,
    holders::context::{
        CACHE, DEFAULT_OBJECT_ADDITIONAL_PROPERTIES, EXTENSION_ANY_ADDITIONAL_PROPERTIES_NAME,
        EXTENSION_FOR_NAME,
    },
    services::references,
    structs::common::{CallStack, ModelName, ParsedSpec},
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

        Script::SpecStart
            .call_with_descriptor(
                out_path,
                &(&openapi.openapi, &openapi.extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_spec_info(out_path, &openapi.info, call_stack)?;
                visit_spec_servers(out_path, &openapi.servers, &openapi.extensions, call_stack)?;
                visit_spec_security(out_path, &openapi.security, &openapi.extensions, call_stack)?;
                visit_spec_tags(out_path, &openapi.tags, &openapi.extensions, call_stack)?;
                visit_external_docs(out_path, &openapi.external_docs, call_stack)?;
                visit_spec_components(
                    out_path,
                    spec_path,
                    Arc::new(spec_as_json),
                    &openapi.components,
                    call_stack,
                )?;
                Ok(())
            })?;

        Script::SpecEnd.call_with_descriptor(
            out_path,
            &(&openapi.openapi, &openapi.extensions),
            call_stack,
        )?;

        if let Some(expected_path) = expected {
            assert_diff(out_path, expected_path)?
        }
        info!("Command execution end for [{:?}]", spec_path);
        Ok(())
    } else {
        Err(anyhow!("Expected a Translate command"))
    }
}

pub fn assert_diff(actual_path: &Path, expected_path: &Path) -> Result<()> {
    let mut test_results: Vec<Result<()>> = Vec::new();
    for entry in fs::read_dir(actual_path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let actual_file = entry.path();
        let expected_file = expected_path.join(&file_name);

        if expected_file.exists() && expected_file.is_file() {
            test_results.push(compare_and_save_diff(
                &actual_file,
                &expected_file,
                actual_path,
            ));
        }
    }

    test_results.iter().for_each(|it| {
        if let Err(err) = it {
            error!("{}", err);
        }
    });

    let total_tests_count = test_results.len();
    let failed_tests_count = test_results.iter().filter(|it| it.is_err()).count();

    if test_results.into_iter().any(|it| it.is_err()) {
        return Err(anyhow!(
            "Found [{}] failed tests from [{}]",
            failed_tests_count,
            total_tests_count
        ));
    }

    Ok(())
}

fn compare_and_save_diff(actual_path: &Path, expected_path: &Path, patch_dir: &Path) -> Result<()> {
    let actual_content = fs::read_to_string(actual_path)?;
    let expected_content = fs::read_to_string(expected_path)?;

    let patch = create_patch(&expected_content, &actual_content);

    let patch_str = format!("{}", patch);

    if patch_str != "--- original\n+++ modified\n" {
        let patch_file_name = format!(
            "{}.patch",
            actual_path
                .file_stem()
                .ok_or_else(|| anyhow!(
                    "File name without extension not found for [{:?}]",
                    actual_path
                ))?
                .to_str()
                .ok_or_else(|| anyhow!(
                    "File name conversion to string error [{:?}]",
                    actual_path
                ))?
        );
        let patch_file_path = patch_dir.join(patch_file_name);

        let mut patch_file = File::create(&patch_file_path)?;
        patch_file.write_all(patch_str.as_bytes())?;
        return Err(anyhow!(
            "\nFailed test with expected value\n[{:?}]\nactual value\n[{:?}]\ndiff saved to\n[{:?}]",
            expected_path,
            actual_path,
            patch_file_path
        ));
    }
    Ok(())
}

pub fn visit_not<T>(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    schema_ref: &ReferenceOr<T>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()>
where
    T: DeserializeOwned + Send + Sync + AsSchemaRef + From<Schema> + 'static,
{
    Script::NotPropertyStart
        .call_with_descriptor(out_path, &(&names_stack, extensions), call_stack)?
        .and_then(|it| visit_schema(parsed_spec, out_path, names_stack, "not", schema_ref, it))?;
    Script::NotPropertyEnd.call_with_descriptor(
        out_path,
        &(&names_stack, extensions),
        call_stack,
    )?;

    Ok(())
}

pub fn visit_schema<T>(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    schema_name: &str,
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
                names_stack,
                schema_name,
                references::resolve_reference::<T>(reference, parsed_spec)?,
                call_stack,
            )?;
        }
        ReferenceOr::Item(schema_item) => {
            let schema_extensions = &schema_item.as_schema().schema_data.extensions;

            let mut current_name_stack = names_stack.to_vec();
            current_name_stack.push(ModelName {
                base: schema_name.to_owned(),
                extended: schema_extensions.get(EXTENSION_FOR_NAME).cloned(),
            });

            let schema_data = &schema_item.as_schema().schema_data;

            Script::SchemaStart
                .call_with_descriptor(
                    out_path,
                    &(&current_name_stack, &schema_data, &schema_extensions),
                    call_stack,
                )?
                .and_then(|it| {
                    visit_discriminator(
                        out_path,
                        &current_name_stack,
                        &schema_data.discriminator,
                        it,
                    )?;
                    visit_external_docs(out_path, &schema_data.external_docs, call_stack)?;
                    visit_generic_example(
                        out_path,
                        &current_name_stack,
                        &schema_data.example,
                        &schema_data.extensions,
                        it,
                    )?;
                    visit_schema_default(
                        out_path,
                        &current_name_stack,
                        &schema_data.default,
                        &schema_data.extensions,
                        it,
                    )?;

                    match &schema_item.as_schema().schema_kind {
                        openapiv3::SchemaKind::Type(type_) => match type_ {
                            openapiv3::Type::Object(object_descriptor) => visit_object(
                                parsed_spec,
                                out_path,
                                &current_name_stack,
                                object_descriptor,
                                schema_extensions,
                                it,
                            ),
                            openapiv3::Type::Array(array_descriptor) => visit_array(
                                parsed_spec,
                                out_path,
                                &current_name_stack,
                                array_descriptor,
                                schema_extensions,
                                it,
                            ),
                            // Simple types
                            openapiv3::Type::String(string_descriptor) => visit_string(
                                out_path,
                                &current_name_stack,
                                string_descriptor,
                                schema_extensions,
                                it,
                            ),
                            openapiv3::Type::Number(number_descriptor) => visit_number(
                                out_path,
                                &current_name_stack,
                                number_descriptor,
                                schema_extensions,
                                it,
                            ),
                            openapiv3::Type::Integer(integer_descriptor) => visit_integer(
                                out_path,
                                &current_name_stack,
                                integer_descriptor,
                                schema_extensions,
                                it,
                            ),
                            openapiv3::Type::Boolean(boolean_descriptor) => visit_boolean(
                                out_path,
                                &current_name_stack,
                                boolean_descriptor,
                                schema_extensions,
                                it,
                            ),
                        },
                        openapiv3::SchemaKind::OneOf { one_of } => visit_one_of(
                            parsed_spec,
                            out_path,
                            &current_name_stack,
                            one_of,
                            schema_extensions,
                            it,
                        ),
                        openapiv3::SchemaKind::AllOf { all_of } => visit_all_of(
                            parsed_spec,
                            out_path,
                            &current_name_stack,
                            all_of,
                            schema_extensions,
                            it,
                        ),
                        openapiv3::SchemaKind::AnyOf { any_of } => visit_any_of(
                            parsed_spec,
                            out_path,
                            &current_name_stack,
                            any_of,
                            schema_extensions,
                            it,
                        ),
                        openapiv3::SchemaKind::Not { not } => visit_not(
                            parsed_spec,
                            out_path,
                            &current_name_stack,
                            not,
                            schema_extensions,
                            it,
                        ),
                        openapiv3::SchemaKind::Any(any_schema) => visit_any_schema(
                            out_path,
                            &current_name_stack,
                            any_schema,
                            schema_extensions,
                            it,
                        ),
                    }
                })?;
            Script::SchemaEnd.call_with_descriptor(
                out_path,
                &(current_name_stack, schema_data, schema_extensions),
                call_stack,
            )?;
        }
    }
    Ok(())
}

pub fn visit_response(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    response_name: &str,
    response_ref: &ReferenceOr<Response>,
    call_stack: &CallStack,
) -> Result<()> {
    match response_ref {
        ReferenceOr::Reference { reference } => {
            visit_response(
                parsed_spec,
                out_path,
                names_stack,
                response_name,
                references::resolve_reference::<Response>(reference, parsed_spec)?,
                call_stack,
            )?;
        }
        ReferenceOr::Item(response) => {
            let response_extensions = &response.extensions;

            let mut current_names_stack = names_stack.to_vec();
            current_names_stack.push(ModelName {
                base: response_name.to_owned(),
                extended: response_extensions.get(EXTENSION_FOR_NAME).cloned(),
            });

            Script::ResponseStart
                .call_with_descriptor(
                    out_path,
                    &(&current_names_stack, response, &response_extensions),
                    call_stack,
                )?
                .and_then(|call_stack| {
                    Script::ResponseHeadersStart
                        .call_with_descriptor(
                            out_path,
                            &(
                                &current_names_stack,
                                &response.headers,
                                &response_extensions,
                            ),
                            call_stack,
                        )?
                        .and_then(|call_stack| {
                            visit_headers(
                                parsed_spec,
                                out_path,
                                &current_names_stack,
                                &response.headers,
                                response_extensions,
                                call_stack,
                            )
                        })?;
                    Script::ResponseHeadersEnd.call_with_descriptor(
                        out_path,
                        &(
                            &current_names_stack,
                            &response.headers,
                            &response_extensions,
                        ),
                        call_stack,
                    )?;

                    Script::ResponseContentStart
                        .call_with_descriptor(
                            out_path,
                            &(
                                &current_names_stack,
                                &response.content,
                                &response_extensions,
                            ),
                            call_stack,
                        )?
                        .and_then(|call_stack| {
                            visit_media_types(
                                parsed_spec,
                                out_path,
                                names_stack,
                                &response.content,
                                call_stack,
                            )
                        })?;
                    Script::ResponseContentEnd.call_with_descriptor(
                        out_path,
                        &(
                            &current_names_stack,
                            &response.content,
                            &response_extensions,
                        ),
                        call_stack,
                    )?;

                    Script::ResponseLinksStart
                        .call_with_descriptor(
                            out_path,
                            &(&current_names_stack, &response.links, &response_extensions),
                            call_stack,
                        )?
                        .and_then(|call_stack| {
                            visit_links(
                                parsed_spec,
                                out_path,
                                names_stack,
                                &response.links,
                                call_stack,
                            )
                        })?;
                    Script::ResponseLinksEnd.call_with_descriptor(
                        out_path,
                        &(&current_names_stack, &response.links, &response_extensions),
                        call_stack,
                    )?;
                    Ok(())
                })?;
            Script::ResponseEnd.call_with_descriptor(
                out_path,
                &(current_names_stack, response, response_extensions),
                call_stack,
            )?;
        }
    }
    Ok(())
}

pub fn visit_string(
    out_path: &Path,
    names_stack: &[ModelName],
    string_descriptor: &StringType,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    Script::StringProperty.call_with_descriptor(
        out_path,
        &(names_stack, string_descriptor, extensions),
        call_stack,
    )?;
    Ok(())
}

pub fn visit_any_schema(
    out_path: &Path,
    names_stack: &[ModelName],
    any_schema_descriptor: &AnySchema,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    Script::AnySchema.call_with_descriptor(
        out_path,
        &(names_stack, any_schema_descriptor, extensions),
        call_stack,
    )?;
    Ok(())
}

pub fn visit_number(
    out_path: &Path,
    names_stack: &[ModelName],
    number_descriptor: &NumberType,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    Script::NumberProperty.call_with_descriptor(
        out_path,
        &(names_stack, number_descriptor, extensions),
        call_stack,
    )?;
    Ok(())
}

pub fn visit_integer(
    out_path: &Path,
    names_stack: &[ModelName],
    integer_descriptor: &IntegerType,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    Script::IntegerProperty.call_with_descriptor(
        out_path,
        &(names_stack, integer_descriptor, extensions),
        call_stack,
    )?;
    Ok(())
}

pub fn visit_array(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    array_descriptor: &ArrayType,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    Script::ArrayPropertyStart
        .call_with_descriptor(
            out_path,
            &(names_stack, array_descriptor, extensions, call_stack),
            call_stack,
        )?
        .and_then(|call_stack| {
            if let Some(it) = &array_descriptor.items {
                visit_schema(parsed_spec, out_path, names_stack, "items", it, call_stack)?;
            }
            Ok(())
        })?;
    Script::ArrayPropertyEnd.call_with_descriptor(
        out_path,
        &(names_stack, array_descriptor, extensions),
        call_stack,
    )?;
    Ok(())
}

pub fn visit_boolean(
    out_path: &Path,
    names_stack: &[ModelName],
    boolean_descriptor: &BooleanType,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    Script::BooleanProperty.call_with_descriptor(
        out_path,
        &(names_stack, boolean_descriptor, extensions),
        call_stack,
    )?;
    Ok(())
}

pub fn visit_one_of(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    schemas: &[ReferenceOr<Schema>],
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !schemas.is_empty() {
        Script::OneOfStart
            .call_with_descriptor(out_path, &(&names_stack, extensions), call_stack)?
            .and_then(|call_stack| {
                schemas.iter().enumerate().try_for_each(|it| {
                    visit_schema(
                        parsed_spec,
                        out_path,
                        names_stack,
                        &format!("oneOf-{}", it.0),
                        it.1,
                        call_stack,
                    )
                })
            })?;
        Script::OneOfEnd.call_with_descriptor(out_path, &(names_stack, extensions), call_stack)?;
    }
    Ok(())
}

pub fn visit_all_of(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    schemas: &[ReferenceOr<Schema>],
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !schemas.is_empty() {
        Script::AllOfStart
            .call_with_descriptor(out_path, &(&names_stack, extensions), call_stack)?
            .and_then(|call_stack| {
                schemas.iter().enumerate().try_for_each(|it| {
                    visit_schema(
                        parsed_spec,
                        out_path,
                        names_stack,
                        &format!("allOf-{}", it.0),
                        it.1,
                        call_stack,
                    )
                })
            })?;
        Script::AllOfEnd.call_with_descriptor(out_path, &(names_stack, extensions), call_stack)?;
    }
    Ok(())
}

pub fn visit_any_of(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    schemas: &[ReferenceOr<Schema>],
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !schemas.is_empty() {
        Script::AnyOfStart
            .call_with_descriptor(out_path, &(&names_stack, extensions), call_stack)?
            .and_then(|call_stack| {
                schemas.iter().enumerate().try_for_each(|it| {
                    visit_schema(
                        parsed_spec,
                        out_path,
                        names_stack,
                        &format!("anyOf-{}", it.0),
                        it.1,
                        call_stack,
                    )
                })
            })?;
        Script::AnyOfEnd.call_with_descriptor(out_path, &(names_stack, extensions), call_stack)?;
    }
    Ok(())
}

pub fn visit_discriminator(
    out_path: &Path,
    names_stack: &[ModelName],
    dicriminator: &Option<Discriminator>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(discriminator) = dicriminator.as_ref() {
        let mut current_names_stack = names_stack.to_vec();
        current_names_stack.push(ModelName {
            base: String::from("discriminator"),
            extended: discriminator.extensions.get(EXTENSION_FOR_NAME).cloned(),
        });

        Script::SchemaDiscriminator.call_with_descriptor(
            out_path,
            &(
                current_names_stack,
                discriminator,
                &discriminator.extensions,
            ),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_generic_example(
    out_path: &Path,
    names_stack: &[ModelName],
    example: &Option<serde_json::Value>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(example) = example {
        Script::GenericExample.call_with_descriptor(
            out_path,
            &(&names_stack, example, extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_generic_parameter(
    out_path: &Path,
    names_stack: &[ModelName],
    parameter_name: &str,
    parameter: &serde_json::Value,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    let mut current_names_stack = names_stack.to_vec();
    current_names_stack.push(ModelName {
        base: parameter_name.to_owned(),
        extended: extensions.get(EXTENSION_FOR_NAME).cloned(),
    });

    Script::GenericParameter.call_with_descriptor(
        out_path,
        &(&current_names_stack, parameter, extensions),
        call_stack,
    )?;
    Ok(())
}

pub fn visit_generic_request_body(
    out_path: &Path,
    names_stack: &[ModelName],
    body: &Option<serde_json::Value>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(body_json) = body {
        Script::GenericRequestBody.call_with_descriptor(
            out_path,
            &(&names_stack, body_json, extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_media_type_encodings(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    encodings: &IndexMap<String, Encoding>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !encodings.is_empty() {
        Script::EncodingsStart
            .call_with_descriptor(out_path, &(&names_stack, encodings, extensions), call_stack)?
            .and_then(|call_stack| {
                encodings.iter().try_for_each(|encoding| {
                    visit_media_type_encoding(
                        parsed_spec,
                        out_path,
                        names_stack,
                        encoding.0,
                        encoding.1,
                        call_stack,
                    )
                })
            })?;
        Script::EncodingsEnd.call_with_descriptor(
            out_path,
            &(&names_stack, encodings, extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_media_type_encoding(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    encoding_name: &str,
    encoding: &Encoding,
    call_stack: &CallStack,
) -> Result<()> {
    let mut current_names_stack = names_stack.to_vec();
    current_names_stack.push(ModelName {
        base: encoding_name.to_owned(),
        extended: encoding.extensions.get(EXTENSION_FOR_NAME).cloned(),
    });

    Script::EncodingStart
        .call_with_descriptor(
            out_path,
            &(&current_names_stack, &encoding, &encoding.extensions),
            call_stack,
        )?
        .and_then(|call_stack| {
            visit_headers(
                parsed_spec,
                out_path,
                &current_names_stack,
                &encoding.headers,
                &encoding.extensions,
                call_stack,
            )
        })?;
    Script::EncodingEnd.call_with_descriptor(
        out_path,
        &(&current_names_stack, &encoding, &encoding.extensions),
        call_stack,
    )?;
    Ok(())
}

pub fn visit_example(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    example_name: &str,
    example_ref: &ReferenceOr<Example>,
    call_stack: &CallStack,
) -> Result<()> {
    match example_ref {
        ReferenceOr::Reference { reference } => {
            visit_example(
                parsed_spec,
                out_path,
                names_stack,
                example_name,
                references::resolve_reference::<Example>(reference, parsed_spec)?,
                call_stack,
            )?;
        }
        ReferenceOr::Item(example) => {
            let mut current_names_stack = names_stack.to_vec();
            current_names_stack.push(ModelName {
                base: example_name.to_owned(),
                extended: example.extensions.get(EXTENSION_FOR_NAME).cloned(),
            });

            Script::ExamplesExample.call_with_descriptor(
                out_path,
                &(&current_names_stack, &example, &example.extensions),
                call_stack,
            )?;
        }
    }
    Ok(())
}

pub fn visit_request_body(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    request_body_name: &str,
    example_ref: &ReferenceOr<RequestBody>,
    call_stack: &CallStack,
) -> Result<()> {
    match example_ref {
        ReferenceOr::Reference { reference } => {
            visit_request_body(
                parsed_spec,
                out_path,
                names_stack,
                request_body_name,
                references::resolve_reference::<RequestBody>(reference, parsed_spec)?,
                call_stack,
            )?;
        }
        ReferenceOr::Item(request_body) => {
            let mut current_names_stack = names_stack.to_vec();
            current_names_stack.push(ModelName {
                base: request_body_name.to_owned(),
                extended: request_body.extensions.get(EXTENSION_FOR_NAME).cloned(),
            });

            Script::RequestBodyStart
                .call_with_descriptor(
                    out_path,
                    &(
                        &current_names_stack,
                        &request_body,
                        &request_body.extensions,
                    ),
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_media_types(
                        parsed_spec,
                        out_path,
                        &current_names_stack,
                        &request_body.content,
                        call_stack,
                    )
                })?;
            Script::RequestBodyEnd.call_with_descriptor(
                out_path,
                &(
                    &current_names_stack,
                    &request_body,
                    &request_body.extensions,
                ),
                call_stack,
            )?;
        }
    }
    Ok(())
}

pub fn visit_parameter_schema_or_content(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    parameter_name: &str,
    parameter_schema_or_content: &ParameterSchemaOrContent,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    Script::ParameterSchemaOrContentStart
        .call_with_descriptor(
            out_path,
            &(&names_stack, parameter_schema_or_content, extensions),
            call_stack,
        )?
        .and_then(|call_stack| {
            match parameter_schema_or_content {
                ParameterSchemaOrContent::Schema(schema_ref) => {
                    visit_schema(
                        parsed_spec,
                        out_path,
                        names_stack,
                        parameter_name,
                        schema_ref,
                        call_stack,
                    )?;
                }
                ParameterSchemaOrContent::Content(media_types) => {
                    visit_media_types(parsed_spec, out_path, names_stack, media_types, call_stack)?;
                }
            }
            Ok(())
        })?;
    Script::ParameterSchemaOrContentEnd.call_with_descriptor(
        out_path,
        &(&names_stack, parameter_schema_or_content, extensions),
        call_stack,
    )?;
    Ok(())
}

pub fn visit_media_types(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    media_types: &IndexMap<String, MediaType>,
    call_stack: &CallStack,
) -> Result<()> {
    media_types.iter().try_for_each(|media_type| {
        visit_media_type(
            parsed_spec,
            out_path,
            names_stack,
            media_type.0,
            media_type.1,
            call_stack,
        )
    })
}

pub fn visit_links(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    links: &IndexMap<String, ReferenceOr<Link>>,
    call_stack: &CallStack,
) -> Result<()> {
    links.iter().try_for_each(|link| {
        visit_link(
            parsed_spec,
            out_path,
            names_stack,
            link.0,
            link.1,
            call_stack,
        )
    })
}

pub fn visit_link(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    link_name: &str,
    link: &ReferenceOr<Link>,
    call_stack: &CallStack,
) -> Result<()> {
    match link {
        ReferenceOr::Reference { reference } => {
            visit_link(
                parsed_spec,
                out_path,
                names_stack,
                link_name,
                references::resolve_reference::<Link>(reference, parsed_spec)?,
                call_stack,
            )?;
        }
        ReferenceOr::Item(link) => {
            let mut current_names_stack = names_stack.to_vec();
            current_names_stack.push(ModelName {
                base: link_name.to_owned(),
                extended: link.extensions.get(EXTENSION_FOR_NAME).cloned(),
            });

            Script::LinkStart
                .call_with_descriptor(
                    out_path,
                    &(&current_names_stack, link, &link.extensions),
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_generic_request_body(
                        out_path,
                        &current_names_stack,
                        &link.request_body,
                        &link.extensions,
                        call_stack,
                    )?;

                    visit_generic_parameters(
                        out_path,
                        &current_names_stack,
                        &link.parameters,
                        &link.extensions,
                        call_stack,
                    )?;

                    if let Some(server) = &link.server {
                        visit_server(out_path, server, call_stack)?;
                    }
                    Ok(())
                })?;
            Script::LinkEnd.call_with_descriptor(
                out_path,
                &(&current_names_stack, link, &link.extensions),
                call_stack,
            )?;
        }
    }
    Ok(())
}

pub fn visit_media_type(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    media_type_name: &str,
    media_type: &MediaType,
    call_stack: &CallStack,
) -> Result<()> {
    let mut current_names_stack = names_stack.to_vec();
    current_names_stack.push(ModelName {
        base: media_type_name.to_owned(),
        extended: media_type.extensions.get(EXTENSION_FOR_NAME).cloned(),
    });

    Script::MediaTypeStart
        .call_with_descriptor(
            out_path,
            &(&current_names_stack, media_type, &media_type.extensions),
            call_stack,
        )?
        .and_then(|call_stack| {
            if let Some(schema_ref) = &media_type.schema {
                visit_schema(
                    parsed_spec,
                    out_path,
                    &current_names_stack,
                    "schema",
                    schema_ref,
                    call_stack,
                )?;
            }

            visit_generic_example(
                out_path,
                &current_names_stack,
                &media_type.example,
                &media_type.extensions,
                call_stack,
            )?;

            visit_examples(
                parsed_spec,
                out_path,
                &current_names_stack,
                &media_type.examples,
                &media_type.extensions,
                call_stack,
            )?;

            visit_media_type_encodings(
                parsed_spec,
                out_path,
                &current_names_stack,
                &media_type.encoding,
                &media_type.extensions,
                call_stack,
            )?;
            Ok(())
        })?;
    Script::MediaTypeEnd.call_with_descriptor(
        out_path,
        &(&current_names_stack, media_type, &media_type.extensions),
        call_stack,
    )?;
    Ok(())
}

pub fn visit_examples(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    examples: &IndexMap<String, ReferenceOr<Example>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !examples.is_empty() {
        Script::ExamplesStart
            .call_with_descriptor(out_path, &(&names_stack, examples, extensions), call_stack)?
            .and_then(|call_stack| {
                examples.iter().try_for_each(|it| {
                    visit_example(parsed_spec, out_path, names_stack, it.0, it.1, call_stack)
                })
            })?;
        Script::ExamplesEnd.call_with_descriptor(
            out_path,
            &(&names_stack, examples, extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_request_bodies(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    request_bodies: &IndexMap<String, ReferenceOr<RequestBody>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !request_bodies.is_empty() {
        Script::RequestBodiesStart
            .call_with_descriptor(
                out_path,
                &(&names_stack, &request_bodies, extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                request_bodies.iter().try_for_each(|it| {
                    visit_request_body(parsed_spec, out_path, names_stack, it.0, it.1, call_stack)
                })
            })?;
        Script::RequestBodiesEnd.call_with_descriptor(
            out_path,
            &(&names_stack, &request_bodies, extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_generic_parameters(
    out_path: &Path,
    names_stack: &[ModelName],
    parameters: &IndexMap<String, serde_json::Value>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !parameters.is_empty() {
        Script::GenericParametersStart
            .call_with_descriptor(
                out_path,
                &(&names_stack, parameters, extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                parameters.iter().try_for_each(|it| {
                    visit_generic_parameter(
                        out_path,
                        names_stack,
                        it.0,
                        it.1,
                        extensions,
                        call_stack,
                    )?;
                    Ok(())
                })
            })?;
        Script::GenericParametersEnd.call_with_descriptor(
            out_path,
            &(&names_stack, parameters, extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_header(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    header_name: &str,
    header: &ReferenceOr<Header>,
    call_stack: &CallStack,
) -> Result<()> {
    match header {
        ReferenceOr::Reference { reference } => {
            visit_header(
                parsed_spec,
                out_path,
                names_stack,
                header_name,
                references::resolve_reference::<Header>(reference, parsed_spec)?,
                call_stack,
            )?;
        }
        ReferenceOr::Item(header) => {
            let mut current_names_stack = names_stack.to_vec();
            current_names_stack.push(ModelName {
                base: header_name.to_owned(),
                extended: header.extensions.get(EXTENSION_FOR_NAME).cloned(),
            });

            Script::HeaderStart
                .call_with_descriptor(
                    out_path,
                    &(&current_names_stack, &header, &header.extensions),
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_parameter_schema_or_content(
                        parsed_spec,
                        out_path,
                        &current_names_stack,
                        "format",
                        &header.format,
                        &header.extensions,
                        call_stack,
                    )?;

                    visit_generic_example(
                        out_path,
                        &current_names_stack,
                        &header.example,
                        &header.extensions,
                        call_stack,
                    )?;

                    visit_examples(
                        parsed_spec,
                        out_path,
                        &current_names_stack,
                        &header.examples,
                        &header.extensions,
                        call_stack,
                    )
                })?;
            Script::HeaderEnd.call_with_descriptor(
                out_path,
                &(&current_names_stack, &header, &header.extensions),
                call_stack,
            )?;
        }
    }
    Ok(())
}

pub fn visit_security_scheme(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    scheme_name: &str,
    security_scheme: &ReferenceOr<SecurityScheme>,
    call_stack: &CallStack,
) -> Result<()> {
    match security_scheme {
        ReferenceOr::Reference { reference } => {
            visit_security_scheme(
                parsed_spec,
                out_path,
                names_stack,
                scheme_name,
                references::resolve_reference::<SecurityScheme>(reference, parsed_spec)?,
                call_stack,
            )?;
        }
        ReferenceOr::Item(security_scheme) => match security_scheme {
            SecurityScheme::APIKey { .. } => {
                visit_security_scheme_apikey(
                    out_path,
                    names_stack,
                    scheme_name,
                    security_scheme,
                    call_stack,
                )?;
            }
            SecurityScheme::HTTP { .. } => {
                visit_security_scheme_http(
                    out_path,
                    names_stack,
                    scheme_name,
                    security_scheme,
                    call_stack,
                )?;
            }
            SecurityScheme::OAuth2 { .. } => {
                visit_security_scheme_oauth2(
                    out_path,
                    names_stack,
                    scheme_name,
                    security_scheme,
                    call_stack,
                )?;
            }
            SecurityScheme::OpenIDConnect { .. } => {
                visit_security_scheme_openid_connect(
                    out_path,
                    names_stack,
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
    names_stack: &[ModelName],
    headers: &IndexMap<String, ReferenceOr<Header>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !headers.is_empty() {
        Script::HeadersStart
            .call_with_descriptor(out_path, &(names_stack, headers, extensions), call_stack)?
            .and_then(|call_stack| {
                headers.iter().try_for_each(|it| {
                    visit_header(parsed_spec, out_path, names_stack, it.0, it.1, call_stack)
                })
            })?;
        Script::HeadersEnd.call_with_descriptor(
            out_path,
            &(names_stack, headers, extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_security_schemes(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    security_schemes: &IndexMap<String, ReferenceOr<SecurityScheme>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !security_schemes.is_empty() {
        Script::SecuritySchemesStart
            .call_with_descriptor(
                out_path,
                &(names_stack, &security_schemes, extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                security_schemes.iter().try_for_each(|it| {
                    visit_security_scheme(
                        parsed_spec,
                        out_path,
                        names_stack,
                        it.0,
                        it.1,
                        call_stack,
                    )
                })
            })?;
        Script::SecuritySchemesEnd.call_with_descriptor(
            out_path,
            &(names_stack, &security_schemes, extensions),
            call_stack,
        )?;
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
        Script::SpecTagsStart
            .call_with_descriptor(out_path, &(tags, extensions), call_stack)?
            .and_then(|call_stack| {
                tags.iter().try_for_each(|tag| {
                    visit_external_docs(out_path, &tag.external_docs, call_stack)?;
                    Script::SpecTag.call_with_descriptor(
                        out_path,
                        &(&tag, &tag.extensions),
                        call_stack,
                    )?;
                    Ok(())
                })
            })?;
        Script::SpecTagsEnd.call_with_descriptor(out_path, &(tags, extensions), call_stack)?;
    }
    Ok(())
}

pub fn visit_spec_security(
    out_path: &Path,
    securities: &Option<Vec<SecurityRequirement>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(it) = securities.as_ref() {
        if !it.is_empty() {
            Script::SpecSecuritiesStart
                .call_with_descriptor(out_path, &(it, extensions), call_stack)?
                .and_then(|call_stack| {
                    it.iter().try_for_each(|sec_map| {
                        Script::SpecSecurity.call_with_descriptor(
                            out_path,
                            &(sec_map, extensions),
                            call_stack,
                        )?;
                        Ok(())
                    })
                })?;
            Script::SpecSecuritiesEnd.call_with_descriptor(
                out_path,
                &(it, extensions),
                call_stack,
            )?;
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
        Script::ExternalDocs.call_with_descriptor(
            out_path,
            &(&it.description, &it.url, &it.extensions),
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
        Script::SchemasStart
            .call_with_descriptor(out_path, &(&extensions), call_stack)?
            .and_then(|it| {
                schemas.iter().try_for_each(|(schema_name, schema_ref)| {
                    visit_schema(parsed_spec, out_path, &[], schema_name, schema_ref, it)
                })
            })?;
        Script::SchemasEnd.call_with_descriptor(out_path, &(&extensions), call_stack)?;
    }
    Ok(())
}

pub fn visit_responses(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    responses: &IndexMap<String, ReferenceOr<Response>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !responses.is_empty() {
        Script::ResponsesStart
            .call_with_descriptor(out_path, &(&extensions), call_stack)?
            .and_then(|it| {
                responses
                    .iter()
                    .try_for_each(|(response_name, response_ref)| {
                        visit_response(parsed_spec, out_path, &[], response_name, response_ref, it)
                    })
            })?;
        Script::ResponsesEnd.call_with_descriptor(out_path, &(&extensions), call_stack)?;
    }
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
        Script::ParametersStart
            .call_with_descriptor(out_path, &(parameters, &extensions), call_stack)?
            .and_then(|call_stack| {
                parameters
                    .iter()
                    .try_for_each(|(parameter_name, parameter_ref)| {
                        visit_parameter(
                            parsed_spec,
                            out_path,
                            &[],
                            parameter_name,
                            parameter_ref,
                            extensions,
                            call_stack,
                        )?;
                        Ok(())
                    })
            })?;
        Script::ParametersEnd.call_with_descriptor(
            out_path,
            &(parameters, &extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_parameter(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
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
                names_stack,
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
                    names_stack,
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
                    names_stack,
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
                    names_stack,
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
                    names_stack,
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
    names_stack: &[ModelName],
    scheme_name: &str,
    http: &SecurityScheme,
    call_stack: &CallStack,
) -> Result<()> {
    if let SecurityScheme::HTTP { extensions, .. } = http {
        let mut current_names_stack = names_stack.to_vec();
        current_names_stack.push(ModelName {
            base: scheme_name.to_owned(),
            extended: extensions.get(EXTENSION_FOR_NAME).cloned(),
        });
        Script::SecuritySchemeHttp.call_with_descriptor(
            out_path,
            &(&current_names_stack, &http, &extensions),
            call_stack,
        )?;
        Ok(())
    } else {
        Err(anyhow!("Not a SecurityScheme.HTTP"))
    }
}

pub fn visit_security_scheme_oauth2(
    out_path: &Path,
    names_stack: &[ModelName],
    scheme_name: &str,
    oauth2: &SecurityScheme,
    call_stack: &CallStack,
) -> Result<()> {
    if let SecurityScheme::OAuth2 {
        flows, extensions, ..
    } = oauth2
    {
        let mut current_names_stack = names_stack.to_vec();
        current_names_stack.push(ModelName {
            base: scheme_name.to_owned(),
            extended: extensions.get(EXTENSION_FOR_NAME).cloned(),
        });
        Script::SecuritySchemeOAuth2Start
            .call_with_descriptor(
                out_path,
                &(&current_names_stack, oauth2, extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_security_scheme_oauth2_flows(
                    out_path,
                    &current_names_stack,
                    flows,
                    call_stack,
                )
            })?;
        Script::SecuritySchemeOAuth2End.call_with_descriptor(
            out_path,
            &(&current_names_stack, oauth2, extensions),
            call_stack,
        )?;
        Ok(())
    } else {
        Err(anyhow!("Not a SecurityScheme.OAuth2"))
    }
}

pub fn visit_security_scheme_oauth2_flows(
    out_path: &Path,
    names_stack: &[ModelName],
    flows: &OAuth2Flows,
    call_stack: &CallStack,
) -> Result<()> {
    Script::SecuritySchemeOAuth2FlowsStart
        .call_with_descriptor(
            out_path,
            &(&names_stack, &flows, &flows.extensions),
            call_stack,
        )?
        .and_then(|call_stack| {
            visit_security_scheme_oauth2_flows_implicit(
                out_path,
                names_stack,
                &flows.implicit,
                call_stack,
            )?;
            visit_security_scheme_oauth2_flows_password(
                out_path,
                names_stack,
                &flows.password,
                call_stack,
            )?;
            visit_security_scheme_oauth2_flows_client_credentials(
                out_path,
                names_stack,
                &flows.client_credentials,
                call_stack,
            )?;
            visit_security_scheme_oauth2_flows_authorization_code(
                out_path,
                names_stack,
                &flows.authorization_code,
                call_stack,
            )
        })?;
    Script::SecuritySchemeOAuth2FlowsEnd.call_with_descriptor(
        out_path,
        &(&names_stack, &flows, &flows.extensions),
        call_stack,
    )?;
    Ok(())
}

pub fn visit_security_scheme_oauth2_flows_implicit(
    out_path: &Path,
    names_stack: &[ModelName],
    flow: &Option<ImplicitOAuth2Flow>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(flow) = flow {
        Script::SecuritySchemeOAuth2FlowImplicit.call_with_descriptor(
            out_path,
            &(&names_stack, &flow, &flow.extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_security_scheme_oauth2_flows_password(
    out_path: &Path,
    names_stack: &[ModelName],
    flow: &Option<PasswordOAuth2Flow>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(flow) = flow {
        Script::SecuritySchemeOAuth2FlowPassword.call_with_descriptor(
            out_path,
            &(&names_stack, &flow, &flow.extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_security_scheme_oauth2_flows_client_credentials(
    out_path: &Path,
    names_stack: &[ModelName],
    flow: &Option<ClientCredentialsOAuth2Flow>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(flow) = flow {
        Script::SecuritySchemeOAuth2FlowClientCredentials.call_with_descriptor(
            out_path,
            &(&names_stack, &flow, &flow.extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_security_scheme_oauth2_flows_authorization_code(
    out_path: &Path,
    names_stack: &[ModelName],
    flow: &Option<AuthorizationCodeOAuth2Flow>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(flow) = flow {
        Script::SecuritySchemeOAuth2FlowAuthorizationCode.call_with_descriptor(
            out_path,
            &(&names_stack, &flow, &flow.extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_security_scheme_apikey(
    out_path: &Path,
    names_stack: &[ModelName],
    scheme_name: &str,
    api_key: &SecurityScheme,
    call_stack: &CallStack,
) -> Result<()> {
    if let SecurityScheme::APIKey { extensions, .. } = api_key {
        let mut current_names_stack = names_stack.to_vec();
        current_names_stack.push(ModelName {
            base: scheme_name.to_owned(),
            extended: extensions.get(EXTENSION_FOR_NAME).cloned(),
        });
        Script::SecuritySchemeApiKey.call_with_descriptor(
            out_path,
            &(&current_names_stack, &api_key, &extensions),
            call_stack,
        )?;
        Ok(())
    } else {
        Err(anyhow!("Not a SecurityScheme.APIKey"))
    }
}

pub fn visit_security_scheme_openid_connect(
    out_path: &Path,
    names_stack: &[ModelName],
    scheme_name: &str,
    openid_connect: &SecurityScheme,
    call_stack: &CallStack,
) -> Result<()> {
    if let SecurityScheme::OpenIDConnect { extensions, .. } = openid_connect {
        let mut current_names_stack = names_stack.to_vec();
        current_names_stack.push(ModelName {
            base: scheme_name.to_owned(),
            extended: extensions.get(EXTENSION_FOR_NAME).cloned(),
        });
        Script::SecuritySchemeOpenIdConnect.call_with_descriptor(
            out_path,
            &(&current_names_stack, &openid_connect, &extensions),
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
    names_stack: &[ModelName],
    parameter_data: &ParameterData,
    call_stack: &CallStack,
) -> Result<()> {
    Script::ParameterDataStart
        .call_with_descriptor(
            out_path,
            &(&names_stack, &parameter_data, &parameter_data.extensions),
            call_stack,
        )?
        .and_then(|call_stack| {
            visit_parameter_schema_or_content(
                parsed_spec,
                out_path,
                names_stack,
                "format",
                &parameter_data.format,
                &parameter_data.extensions,
                call_stack,
            )?;
            visit_generic_example(
                out_path,
                names_stack,
                &parameter_data.example,
                &parameter_data.extensions,
                call_stack,
            )?;
            visit_examples(
                parsed_spec,
                out_path,
                names_stack,
                &parameter_data.examples,
                &parameter_data.extensions,
                call_stack,
            )?;
            Ok(())
        })?;
    Script::ParameterDataEnd.call_with_descriptor(
        out_path,
        &(&names_stack, &parameter_data, &parameter_data.extensions),
        call_stack,
    )?;
    Ok(())
}

pub fn visit_query_parameter(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    parameter_name: &str,
    parameter: &Parameter,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Parameter::Query { parameter_data, .. } = parameter {
        let mut current_names_stack = names_stack.to_vec();
        current_names_stack.push(ModelName {
            base: parameter_name.to_owned(),
            extended: extensions.get(EXTENSION_FOR_NAME).cloned(),
        });

        Script::QueryParameterStart
            .call_with_descriptor(
                out_path,
                &(&current_names_stack, &parameter, &extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_parameter_data(
                    parsed_spec,
                    out_path,
                    &current_names_stack,
                    parameter_data,
                    call_stack,
                )?;
                Ok(())
            })?;
        Script::QueryParameterEnd.call_with_descriptor(
            out_path,
            &(&current_names_stack, &parameter, &extensions),
            call_stack,
        )?;
        Ok(())
    } else {
        Err(anyhow!("Not a Query parameter"))
    }
}

pub fn visit_header_parameter(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    parameter_name: &str,
    parameter: &Parameter,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Parameter::Header { parameter_data, .. } = parameter {
        let mut current_names_stack = names_stack.to_vec();
        current_names_stack.push(ModelName {
            base: parameter_name.to_owned(),
            extended: extensions.get(EXTENSION_FOR_NAME).cloned(),
        });

        Script::HeaderParameterStart
            .call_with_descriptor(
                out_path,
                &(&current_names_stack, &parameter, &extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_parameter_data(
                    parsed_spec,
                    out_path,
                    &current_names_stack,
                    parameter_data,
                    call_stack,
                )?;
                Ok(())
            })?;
        Script::HeaderParameterEnd.call_with_descriptor(
            out_path,
            &(&current_names_stack, &parameter, &extensions),
            call_stack,
        )?;
        Ok(())
    } else {
        Err(anyhow!("Not a Header parameter"))
    }
}

pub fn visit_path_parameter(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    parameter_name: &str,
    parameter: &Parameter,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Parameter::Path { parameter_data, .. } = parameter {
        let mut current_names_stack = names_stack.to_vec();
        current_names_stack.push(ModelName {
            base: parameter_name.to_owned(),
            extended: extensions.get(EXTENSION_FOR_NAME).cloned(),
        });

        Script::PathParameterStart
            .call_with_descriptor(
                out_path,
                &(&current_names_stack, &parameter, &extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_parameter_data(
                    parsed_spec,
                    out_path,
                    &current_names_stack,
                    parameter_data,
                    call_stack,
                )?;
                Ok(())
            })?;
        Script::PathParameterEnd.call_with_descriptor(
            out_path,
            &(&current_names_stack, &parameter, &extensions),
            call_stack,
        )?;
        Ok(())
    } else {
        Err(anyhow!("Not a Path parameter"))
    }
}

pub fn visit_cookie_parameter(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    parameter_name: &str,
    parameter: &Parameter,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Parameter::Cookie { parameter_data, .. } = parameter {
        let mut current_names_stack = names_stack.to_vec();
        current_names_stack.push(ModelName {
            base: parameter_name.to_owned(),
            extended: extensions.get(EXTENSION_FOR_NAME).cloned(),
        });

        Script::CookieParameterStart
            .call_with_descriptor(
                out_path,
                &(&current_names_stack, &parameter, &extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_parameter_data(
                    parsed_spec,
                    out_path,
                    &current_names_stack,
                    parameter_data,
                    call_stack,
                )?;
                Ok(())
            })?;
        Script::CookieParameterEnd.call_with_descriptor(
            out_path,
            &(&current_names_stack, &parameter, &extensions),
            call_stack,
        )?;
        Ok(())
    } else {
        Err(anyhow!("Not a Cookie parameter"))
    }
}

pub fn visit_spec_components(
    out_path: &Path,
    spec_path: &Path,
    spec_as_json: Arc<serde_json::Value>,
    components: &Option<Components>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(components) = components {
        let parsed_spec = ParsedSpec {
            path: spec_path.to_owned(),
            spec: spec_as_json,
        };

        visit_schemas(
            &parsed_spec,
            out_path,
            &components.schemas,
            &components.extensions,
            call_stack,
        )?;

        visit_responses(
            &parsed_spec,
            out_path,
            &components.responses,
            &components.extensions,
            call_stack,
        )?;

        visit_parameters(
            &parsed_spec,
            out_path,
            &components.parameters,
            &components.extensions,
            call_stack,
        )?;

        visit_examples(
            &parsed_spec,
            out_path,
            &[],
            &components.examples,
            &components.extensions,
            call_stack,
        )?;

        visit_request_bodies(
            &parsed_spec,
            out_path,
            &[],
            &components.request_bodies,
            &components.extensions,
            call_stack,
        )?;

        visit_headers(
            &parsed_spec,
            out_path,
            &[],
            &components.headers,
            &components.extensions,
            call_stack,
        )?;

        visit_security_schemes(
            &parsed_spec,
            out_path,
            &[],
            &components.security_schemes,
            &components.extensions,
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_schema_default(
    out_path: &Path,
    names_stack: &[ModelName],
    default: &Option<serde_json::Value>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(default) = default.as_ref() {
        let mut current_names_stack = names_stack.to_vec();
        current_names_stack.push(ModelName {
            base: String::from("default"),
            extended: None,
        });

        Script::SchemaDefault.call_with_descriptor(
            out_path,
            &(current_names_stack, default, extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_spec_info(out_path: &Path, info: &Info, call_stack: &CallStack) -> Result<()> {
    Script::SpecInfoStart
        .call_with_descriptor(out_path, &(&info, &info.extensions), call_stack)?
        .and_then(|call_stack| {
            visit_spec_info_contact(out_path, &info.contact, call_stack)?;
            visit_spec_info_license(out_path, &info.license, call_stack)
        })?;
    Script::SpecInfoEnd.call_with_descriptor(out_path, &(&info, &info.extensions), call_stack)?;
    Ok(())
}

pub fn visit_spec_info_contact(
    out_path: &Path,
    contact: &Option<Contact>,
    call_stack: &CallStack,
) -> Result<()> {
    if let Some(it) = contact {
        Script::SpecInfoContact.call_with_descriptor(
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
        Script::SpecInfoLicense.call_with_descriptor(
            out_path,
            &(&it, &it.extensions),
            call_stack,
        )?;
    }
    Ok(())
}
pub fn visit_server(out_path: &Path, server: &Server, call_stack: &CallStack) -> Result<()> {
    Script::ServerStart
        .call_with_descriptor(out_path, &(&server, &server.extensions), call_stack)?
        .and_then(|call_stack| {
            if let Some(variables) = server.variables.as_ref() {
                variables.iter().try_for_each(|it| {
                    Script::ServerVariable.call_with_descriptor(
                        out_path,
                        &(&server.url, &it.0, &it.1, &it.1.extensions),
                        call_stack,
                    )?;
                    Ok(())
                })?;
            }
            Ok(())
        })?;
    Script::ServerEnd.call_with_descriptor(out_path, &(&server, &server.extensions), call_stack)?;
    Ok(())
}

pub fn visit_spec_servers(
    out_path: &Path,
    servers: &Vec<Server>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    if !servers.is_empty() {
        Script::SpecServersStart
            .call_with_descriptor(out_path, &(servers, extensions), call_stack)?
            .and_then(|call_stack| {
                servers.iter().try_for_each(|server| {
                    visit_server(out_path, server, call_stack)?;
                    Ok(())
                })
            })?;
        Script::SpecServersEnd.call_with_descriptor(
            out_path,
            &(servers, extensions),
            call_stack,
        )?;
    }
    Ok(())
}

pub fn visit_object_property<T>(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    property_name: &str,
    property_schema_ref: &ReferenceOr<T>,
    extensions: &IndexMap<String, serde_json::Value>,
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
                names_stack,
                property_name,
                references::resolve_reference::<T>(reference, parsed_spec)?,
                extensions,
                call_stack,
            )?;
            Ok(())
        }
        ReferenceOr::Item(schema) => {
            let schema = schema.as_schema();

            let mut current_names_stack = names_stack.to_vec();
            current_names_stack.push(ModelName {
                base: property_name.to_owned(),
                extended: schema
                    .schema_data
                    .extensions
                    .get(EXTENSION_FOR_NAME)
                    .cloned(),
            });

            Script::ObjectPropertyStart
                .call_with_descriptor(
                    out_path,
                    &(&current_names_stack, schema, extensions),
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_schema(
                        parsed_spec,
                        out_path,
                        names_stack,
                        property_name,
                        property_schema_ref,
                        call_stack,
                    )
                })?;
            Script::ObjectPropertyEnd.call_with_descriptor(
                out_path,
                &(current_names_stack, schema, extensions),
                call_stack,
            )?;
            Ok(())
        }
    }
}

pub fn visit_object(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: &[ModelName],
    object_description: &ObjectType,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<()> {
    Script::ObjectStart
        .call_with_descriptor(
            out_path,
            &(names_stack, object_description, extensions),
            call_stack,
        )?
        .and_then(|call_stack| {
            Script::ObjectPropertiesStart
                .call_with_descriptor(
                    out_path,
                    &(names_stack, &object_description.properties, extensions),
                    call_stack,
                )?
                .and_then(|call_stack| {
                    object_description.properties.iter().try_for_each(
                        |(local_property_name, property_schema_ref)| -> Result<()> {
                            visit_object_property(
                                parsed_spec,
                                out_path,
                                names_stack,
                                local_property_name,
                                property_schema_ref,
                                extensions,
                                call_stack,
                            )
                        },
                    )?;
                    Ok(())
                })?;
            Script::ObjectPropertiesEnd.call_with_descriptor(
                out_path,
                &(names_stack, &object_description.properties, extensions),
                call_stack,
            )?;

            let mut current_names_stack = names_stack.to_vec();
            //AdditionalProperties it is just especial one property
            current_names_stack.push(ModelName {
                base: DEFAULT_OBJECT_ADDITIONAL_PROPERTIES.to_owned(),
                // additionalProperties does not have schema, so extensions to it sent from object level
                extended: extensions
                    .get(EXTENSION_ANY_ADDITIONAL_PROPERTIES_NAME)
                    .cloned(),
            });

            if let Some(it) = object_description.additional_properties.as_ref() {
                match it {
                    openapiv3::AdditionalProperties::Any(value) => {
                        Script::ObjectAdditionalPropertiesAny.call_with_descriptor(
                            out_path,
                            &(
                                &current_names_stack,
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
                        Script::ObjectAdditionalPropertiesStart
                            .call_with_descriptor(
                                out_path,
                                &(
                                    &current_names_stack,
                                    schema_ref,
                                    object_description.min_properties,
                                    object_description.max_properties,
                                    extensions,
                                ),
                                call_stack,
                            )?
                            .and_then(|call_stack| {
                                visit_schema(
                                    parsed_spec,
                                    out_path,
                                    names_stack,
                                    DEFAULT_OBJECT_ADDITIONAL_PROPERTIES,
                                    schema_ref,
                                    call_stack,
                                )
                            })?;
                        Script::ObjectAdditionalPropertiesEnd.call_with_descriptor(
                            out_path,
                            &(
                                &current_names_stack,
                                schema_ref,
                                object_description.min_properties,
                                object_description.max_properties,
                                extensions,
                            ),
                            call_stack,
                        )?;
                    }
                }
            }
            Ok(())
        })?;
    Script::ObjectEnd.call_with_descriptor(
        out_path,
        &(names_stack, object_description, extensions),
        call_stack,
    )?;
    Ok(())
}
