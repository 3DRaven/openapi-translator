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
    structs::common::{get_call_id, BracketScripts, CallStack, ParsedSpec},
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
                spec_path.to_str(),
                out_path,
                &(&openapi.openapi, &openapi.extensions),
                call_stack,
            )?
            .and_then(|call_stack| visit_spec_info(out_path, &openapi.info, call_stack))?
            .and_then(|call_stack| {
                visit_servers(out_path, &openapi.servers, &openapi.extensions, call_stack)
            })?
            .and_then(|call_stack| visit_paths(&parsed_spec, out_path, &openapi.paths, call_stack))?
            .and_then(|call_stack| {
                visit_security_requirements(
                    out_path,
                    &openapi.security,
                    &openapi.extensions,
                    call_stack,
                )
            })?
            .and_then(|call_stack| {
                visit_spec_tags(out_path, &openapi.tags, &openapi.extensions, call_stack)
            })?
            .and_then(|call_stack| {
                visit_external_docs(out_path, &openapi.external_docs, call_stack)
            })?
            .and_then(|call_stack| {
                visit_spec_components(&parsed_spec, out_path, &openapi.components, call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitSpecEnd.call_with_descriptor(
                    spec_path.to_str(),
                    out_path,
                    &(&openapi.openapi, &openapi.extensions),
                    call_stack,
                )
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

pub fn visit_not(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    schema_ref: &ReferenceOr<Schema>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    Script::VisitPropertyNotStart
        .call_with_descriptor(None, out_path, &(schema_ref, extensions), call_stack)?
        .and_then(|call_stack| visit_schema(parsed_spec, out_path, None, schema_ref, call_stack))?
        .and_then(|call_stack| {
            Script::VisitPropertyNotEnd.call_with_descriptor(
                None,
                out_path,
                &(schema_ref, extensions),
                call_stack,
            )
        })
}

pub fn visit_schema(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    schema_name: Option<&str>,
    schema_ref: &ReferenceOr<Schema>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    match schema_ref {
        ReferenceOr::Reference { reference } => {
            let schema = references::resolve_reference::<Schema>(reference, parsed_spec)?;
            Script::VisitSchemaReference
                .call_with_descriptor(
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
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_schema(parsed_spec, out_path, None, schema, call_stack)
                })
        }
        ReferenceOr::Item(schema_item) => {
            let schema_extensions = &schema_item.as_schema().schema_data.extensions;

            let schema_data = &schema_item.as_schema().schema_data;

            Script::VisitSchemaStart
                .call_with_descriptor(
                    schema_name,
                    out_path,
                    &(schema_name, &schema_data, &schema_extensions),
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_discriminator(out_path, &schema_data.discriminator, call_stack)
                })?
                .and_then(|call_stack| {
                    visit_external_docs(out_path, &schema_data.external_docs, call_stack)
                })?
                .and_then(|call_stack| {
                    visit_generic_example(
                        out_path,
                        &schema_data.example,
                        &schema_data.extensions,
                        call_stack,
                    )
                })?
                .and_then(|call_stack| {
                    visit_schema_default(
                        out_path,
                        &schema_data.default,
                        &schema_data.extensions,
                        call_stack,
                    )
                })?
                .and_then(|call_stack| {
                    match &schema_item.as_schema().schema_kind {
                        openapiv3::SchemaKind::Type(type_) => match type_ {
                            openapiv3::Type::Object(object_descriptor) => visit_object(
                                parsed_spec,
                                out_path,
                                object_descriptor,
                                schema_extensions,
                                call_stack,
                            ),
                            openapiv3::Type::Array(array_descriptor) => visit_array(
                                parsed_spec,
                                out_path,
                                array_descriptor,
                                schema_extensions,
                                call_stack,
                            ),
                            // Simple types
                            openapiv3::Type::String(string_descriptor) => visit_string(
                                out_path,
                                string_descriptor,
                                schema_extensions,
                                call_stack,
                            ),
                            openapiv3::Type::Number(number_descriptor) => visit_number(
                                out_path,
                                number_descriptor,
                                schema_extensions,
                                call_stack,
                            ),
                            openapiv3::Type::Integer(integer_descriptor) => visit_integer(
                                out_path,
                                integer_descriptor,
                                schema_extensions,
                                call_stack,
                            ),
                            openapiv3::Type::Boolean(boolean_descriptor) => visit_boolean(
                                out_path,
                                boolean_descriptor,
                                schema_extensions,
                                call_stack,
                            ),
                        },
                        openapiv3::SchemaKind::OneOf { one_of } => visit_group_of(
                            parsed_spec,
                            out_path,
                            one_of,
                            &BracketScripts {
                                start: Script::VisitOneOfStart,
                                end: Script::VisitOneOfEnd,
                            },
                            schema_extensions,
                            call_stack,
                        ),
                        openapiv3::SchemaKind::AllOf { all_of } => visit_group_of(
                            parsed_spec,
                            out_path,
                            all_of,
                            &BracketScripts {
                                start: Script::VisitAllOfStart,
                                end: Script::VisitAllOfEnd,
                            },
                            schema_extensions,
                            call_stack,
                        ),
                        openapiv3::SchemaKind::AnyOf { any_of } => visit_group_of(
                            parsed_spec,
                            out_path,
                            any_of,
                            &BracketScripts {
                                start: Script::VisitAnyOfStart,
                                end: Script::VisitAnyOfEnd,
                            },
                            schema_extensions,
                            call_stack,
                        ),
                        openapiv3::SchemaKind::Not { not } => {
                            let unboxed = not.as_ref();
                            visit_not(
                                parsed_spec,
                                out_path,
                                unboxed,
                                schema_extensions,
                                call_stack,
                            )
                        }
                        openapiv3::SchemaKind::Any(any_schema) => visit_any_schema(
                            parsed_spec,
                            out_path,
                            any_schema,
                            schema_extensions,
                            call_stack,
                        ),
                    }
                })?
                .and_then(|call_stack| {
                    Script::VisitSchemaEnd.call_with_descriptor(
                        schema_name,
                        out_path,
                        &(schema_name, schema_data, schema_extensions),
                        call_stack,
                    )
                })
        }
    }
}

pub fn visit_response(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    response_name: Option<&str>,
    response_ref: &ReferenceOr<Response>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    match response_ref {
        ReferenceOr::Reference { reference } => {
            let response = references::resolve_reference::<Response>(reference, parsed_spec)?;
            Script::VisitResponseReference
                .call_with_descriptor(
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
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_response(parsed_spec, out_path, None, response, call_stack)
                })
        }
        ReferenceOr::Item(response) => {
            let response_extensions = &response.extensions;

            Script::VisitResponseStart
                .call_with_descriptor(
                    response_name,
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
                    )
                })?
                .and_then(|call_stack| {
                    visit_media_types(
                        parsed_spec,
                        out_path,
                        &response.content,
                        response_extensions,
                        call_stack,
                    )
                })?
                .and_then(|call_stack| {
                    visit_links(
                        parsed_spec,
                        out_path,
                        &response.links,
                        response_extensions,
                        call_stack,
                    )
                })?
                .and_then(|call_stack| {
                    Script::VisitResponseEnd.call_with_descriptor(
                        response_name,
                        out_path,
                        &(response_name, response, response_extensions),
                        call_stack,
                    )
                })
        }
    }
}

pub fn visit_string(
    out_path: &Path,
    string_descriptor: &StringType,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    Script::VisitStringProperty.call_with_descriptor(
        None,
        out_path,
        &(string_descriptor, extensions),
        call_stack,
    )
}

pub fn visit_any_schema(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    any_schema_descriptor: &AnySchema,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    Script::VisitAnySchemaStart
        .call_with_descriptor(
            any_schema_descriptor.typ.as_deref(),
            out_path,
            &(any_schema_descriptor, extensions),
            call_stack,
        )?
        .and_then(|call_stack| {
            if !any_schema_descriptor.all_of.is_empty() {
                visit_group_of(
                    parsed_spec,
                    out_path,
                    &any_schema_descriptor.all_of,
                    &BracketScripts {
                        start: Script::VisitAllOfStart,
                        end: Script::VisitAllOfEnd,
                    },
                    extensions,
                    call_stack,
                )
            } else {
                Ok(call_stack.clone())
            }
        })?
        .and_then(|call_stack| {
            if !any_schema_descriptor.any_of.is_empty() {
                visit_group_of(
                    parsed_spec,
                    out_path,
                    &any_schema_descriptor.any_of,
                    &BracketScripts {
                        start: Script::VisitAnyOfStart,
                        end: Script::VisitAnyOfEnd,
                    },
                    extensions,
                    call_stack,
                )
            } else {
                Ok(call_stack.clone())
            }
        })?
        .and_then(|call_stack| {
            if !any_schema_descriptor.one_of.is_empty() {
                visit_group_of(
                    parsed_spec,
                    out_path,
                    &any_schema_descriptor.one_of,
                    &BracketScripts {
                        start: Script::VisitOneOfStart,
                        end: Script::VisitOneOfEnd,
                    },
                    extensions,
                    call_stack,
                )
            } else {
                Ok(call_stack.clone())
            }
        })?
        .and_then(|call_stack| {
            if let Some(schema) = any_schema_descriptor.not.as_ref() {
                visit_not(parsed_spec, out_path, schema, extensions, call_stack)
            } else {
                Ok(call_stack.clone())
            }
        })?
        .and_then(|call_stack| {
            // visit_object(parsed_spec, out_path, object_description, extensions, call_stack)
            if let Some(schema) = any_schema_descriptor.typ.as_ref() {
                match schema.as_str() {
                    "string" => visit_string(
                        out_path,
                        &serde_json::from_value(serde_json::to_value(any_schema_descriptor)?)?,
                        extensions,
                        call_stack,
                    ),
                    "number" => visit_number(
                        out_path,
                        &serde_json::from_value(serde_json::to_value(any_schema_descriptor)?)?,
                        extensions,
                        call_stack,
                    ),
                    "integer" => visit_integer(
                        out_path,
                        &serde_json::from_value(serde_json::to_value(any_schema_descriptor)?)?,
                        extensions,
                        call_stack,
                    ),
                    "boolean" => visit_boolean(
                        out_path,
                        &serde_json::from_value(serde_json::to_value(any_schema_descriptor)?)?,
                        extensions,
                        call_stack,
                    ),
                    "array" => visit_array(
                        parsed_spec,
                        out_path,
                        &serde_json::from_value(serde_json::to_value(any_schema_descriptor)?)?,
                        extensions,
                        call_stack,
                    ),
                    "object" => visit_object(
                        parsed_spec,
                        out_path,
                        &serde_json::from_value(serde_json::to_value(any_schema_descriptor)?)?,
                        extensions,
                        call_stack,
                    ),
                    _ => Ok(call_stack.clone()),
                }
            } else {
                Ok(call_stack.clone())
            }
        })?
        .and_then(|call_stack| {
            Script::VisitAnySchemaEnd.call_with_descriptor(
                any_schema_descriptor.typ.as_deref(),
                out_path,
                &(any_schema_descriptor, extensions),
                call_stack,
            )
        })
}

pub fn visit_number(
    out_path: &Path,
    number_descriptor: &NumberType,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    Script::VisitNumberProperty.call_with_descriptor(
        None,
        out_path,
        &(number_descriptor, extensions),
        call_stack,
    )
}

pub fn visit_integer(
    out_path: &Path,
    integer_descriptor: &IntegerType,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    Script::VisitIntegerProperty.call_with_descriptor(
        None,
        out_path,
        &(integer_descriptor, extensions),
        call_stack,
    )
}

pub fn visit_array(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    array_descriptor: &ArrayType,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    Script::VisitArrayPropertyStart
        .call_with_descriptor(
            None,
            out_path,
            &(array_descriptor, extensions, call_stack),
            call_stack,
        )?
        .and_then(|call_stack| {
            if let Some(it) = &array_descriptor.items {
                let unboxed = it.clone().unbox();
                visit_schema(parsed_spec, out_path, None, &unboxed, call_stack)
            } else {
                Ok(call_stack.clone())
            }
        })?
        .and_then(|call_stack| {
            Script::VisitArrayPropertyEnd.call_with_descriptor(
                None,
                out_path,
                &(array_descriptor, extensions),
                call_stack,
            )
        })
}

pub fn visit_boolean(
    out_path: &Path,
    boolean_descriptor: &BooleanType,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    Script::VisitBooleanProperty.call_with_descriptor(
        None,
        out_path,
        &(boolean_descriptor, extensions),
        call_stack,
    )
}

pub fn visit_group_of(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    schemas: &[ReferenceOr<Schema>],
    braced_scripts: &BracketScripts,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if !schemas.is_empty() {
        braced_scripts
            .start
            .call_with_descriptor(None, out_path, &(schemas, &extensions), call_stack)?
            .and_then(|call_stack| {
                let mut current_call_stack = call_stack.clone();
                for schema in schemas {
                    current_call_stack =
                        visit_schema(parsed_spec, out_path, None, schema, &current_call_stack)?;
                }
                Ok(current_call_stack)
            })?
            .and_then(|call_stack| {
                braced_scripts.end.call_with_descriptor(
                    None,
                    out_path,
                    &(schemas, extensions),
                    call_stack,
                )
            })
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_discriminator(
    out_path: &Path,
    dicriminator: &Option<Discriminator>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let Some(discriminator) = dicriminator.as_ref() {
        Script::VisitDiscriminator.call_with_descriptor(
            Some(&discriminator.property_name),
            out_path,
            &(discriminator, &discriminator.extensions),
            call_stack,
        )
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_generic_example(
    out_path: &Path,
    example: &Option<serde_json::Value>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let Some(example) = example {
        Script::VisitGenericExample.call_with_descriptor(
            None,
            out_path,
            &(&example, extensions),
            call_stack,
        )
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_generic_parameter(
    out_path: &Path,
    parameter_name: &str,
    parameter: &serde_json::Value,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    Script::VisitGenericParameter.call_with_descriptor(
        Some(parameter_name),
        out_path,
        &(parameter_name, parameter, extensions),
        call_stack,
    )
}

pub fn visit_generic_request_body(
    out_path: &Path,
    body: &Option<serde_json::Value>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let Some(body_json) = body {
        Script::VisitGenericRequestBody.call_with_descriptor(
            None,
            out_path,
            &(&body_json, extensions),
            call_stack,
        )
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_media_type_encodings(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    encodings: &IndexMap<String, Encoding>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if !encodings.is_empty() {
        Script::VisitEncodingsStart
            .call_with_descriptor(None, out_path, &(&encodings, extensions), call_stack)?
            .and_then(|call_stack| {
                let mut current_call_stack = call_stack.clone();
                for encoding in encodings {
                    current_call_stack = visit_media_type_encoding(
                        parsed_spec,
                        out_path,
                        encoding.0,
                        encoding.1,
                        &current_call_stack,
                    )?;
                }
                Ok(current_call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitEncodingsEnd.call_with_descriptor(
                    None,
                    out_path,
                    &(&encodings, extensions),
                    call_stack,
                )
            })
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_media_type_encoding(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    encoding_name: &str,
    encoding: &Encoding,
    call_stack: &CallStack,
) -> Result<CallStack> {
    Script::VisitEncodingStart
        .call_with_descriptor(
            Some(encoding_name),
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
                Some(encoding_name),
                out_path,
                &(&encoding_name, &encoding, &encoding.extensions),
                call_stack,
            )
        })
}

pub fn visit_example(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    example_name: Option<&str>,
    example_ref: &ReferenceOr<Example>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    match example_ref {
        ReferenceOr::Reference { reference } => {
            let example = references::resolve_reference::<Example>(reference, parsed_spec)?;
            Script::VisitExampleReference
                .call_with_descriptor(
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
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_example(parsed_spec, out_path, None, example, call_stack)
                })
        }
        ReferenceOr::Item(example) => Script::VisitExampleStart
            .call_with_descriptor(
                example_name,
                out_path,
                &(example_name, &example, &example.extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_generic_example(out_path, &example.value, &example.extensions, call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitExampleEnd.call_with_descriptor(
                    example_name,
                    out_path,
                    &(example_name, &example, &example.extensions),
                    call_stack,
                )
            }),
    }
}

pub fn visit_request_body(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    request_body_name: Option<&str>,
    request_body_ref: &ReferenceOr<RequestBody>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    match request_body_ref {
        ReferenceOr::Reference { reference } => {
            let request_body =
                references::resolve_reference::<RequestBody>(reference, parsed_spec)?;
            Script::VisitRequestBodyReference
                .call_with_descriptor(
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
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_request_body(parsed_spec, out_path, None, request_body, call_stack)
                })
        }
        ReferenceOr::Item(request_body) => Script::VisitRequestBodyStart
            .call_with_descriptor(
                request_body_name,
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
                    request_body_name,
                    out_path,
                    &(&request_body_name, &request_body, &request_body.extensions),
                    call_stack,
                )
            }),
    }
}

pub fn visit_parameter_schema_or_content(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameter_name: Option<&str>,
    parameter_schema_or_content: &ParameterSchemaOrContent,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    Script::VisitParameterSchemaOrContentStart
        .call_with_descriptor(
            parameter_name,
            out_path,
            &(&parameter_name, &parameter_schema_or_content, extensions),
            call_stack,
        )?
        .and_then(|call_stack| match parameter_schema_or_content {
            ParameterSchemaOrContent::Schema(schema_ref) => {
                visit_schema(parsed_spec, out_path, None, schema_ref, call_stack)
            }
            ParameterSchemaOrContent::Content(media_types) => {
                visit_media_types(parsed_spec, out_path, media_types, extensions, call_stack)
            }
        })?
        .and_then(|call_stack| {
            Script::VisitParameterSchemaOrContentEnd.call_with_descriptor(
                parameter_name,
                out_path,
                &(&parameter_name, &parameter_schema_or_content, extensions),
                call_stack,
            )
        })
}

pub fn visit_media_types(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    media_types: &IndexMap<String, MediaType>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if !media_types.is_empty() {
        Script::VisitMediaTypesStart
            .call_with_descriptor(None, out_path, &(media_types, extensions), call_stack)?
            .and_then(|call_stack| {
                let mut current_call_stack = call_stack.clone();
                for media_type in media_types {
                    current_call_stack = visit_media_type(
                        parsed_spec,
                        out_path,
                        media_type.0,
                        media_type.1,
                        &current_call_stack,
                    )?;
                }
                Ok(current_call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitMediaTypesEnd.call_with_descriptor(
                    None,
                    out_path,
                    &(media_types, extensions),
                    call_stack,
                )
            })
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_callbacks(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    operation_callbacks: &IndexMap<String, ReferenceOr<Callback>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if !operation_callbacks.is_empty() {
        Script::VisitAsyncCallbacksStart
            .call_with_descriptor(
                None,
                out_path,
                &(operation_callbacks, &extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                let mut current_call_stack = call_stack.clone();
                for callbacks in operation_callbacks {
                    current_call_stack = visit_callback(
                        parsed_spec,
                        out_path,
                        Some(callbacks.0),
                        callbacks.1,
                        extensions,
                        &current_call_stack,
                    )?;
                }
                Ok(current_call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitAsyncCallbacksEnd.call_with_descriptor(
                    None,
                    out_path,
                    &(operation_callbacks, &extensions),
                    call_stack,
                )
            })
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_links(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    links: &IndexMap<String, ReferenceOr<Link>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if !links.is_empty() {
        Script::VisitLinksStart
            .call_with_descriptor(None, out_path, &(links, &extensions), call_stack)?
            .and_then(|call_stack| {
                let mut current_call_stack = call_stack.clone();
                for link in links {
                    current_call_stack = visit_link(
                        parsed_spec,
                        out_path,
                        Some(link.0),
                        link.1,
                        &current_call_stack,
                    )?;
                }
                Ok(current_call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitLinksEnd.call_with_descriptor(
                    None,
                    out_path,
                    &(links, &extensions),
                    call_stack,
                )
            })
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_link(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    link_name: Option<&str>,
    link_ref: &ReferenceOr<Link>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    match link_ref {
        ReferenceOr::Reference { reference } => {
            let link = references::resolve_reference::<Link>(reference, parsed_spec)?;
            Script::VisitLinkReference
                .call_with_descriptor(
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
                    call_stack,
                )?
                .and_then(|call_stack| visit_link(parsed_spec, out_path, None, link, call_stack))
        }
        ReferenceOr::Item(link) => Script::VisitLinkStart
            .call_with_descriptor(
                link_name,
                out_path,
                &(link_name, link, &link.extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_generic_request_body(
                    out_path,
                    &link.request_body,
                    &link.extensions,
                    call_stack,
                )
            })?
            .and_then(|call_stack| {
                visit_generic_parameters(out_path, &link.parameters, &link.extensions, call_stack)
            })?
            .and_then(|call_stack| {
                if let Some(server) = &link.server {
                    visit_server(out_path, server, call_stack)
                } else {
                    Ok(call_stack.clone())
                }
            })?
            .and_then(|call_stack| {
                Script::VisitLinkEnd.call_with_descriptor(
                    link_name,
                    out_path,
                    &(link_name, link, &link.extensions),
                    call_stack,
                )
            }),
    }
}

pub fn visit_callback(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    callbacks_name: Option<&str>,
    callbacks: &ReferenceOr<Callback>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    match callbacks {
        ReferenceOr::Reference { reference } => {
            let callback = references::resolve_reference::<Callback>(reference, parsed_spec)?;
            Script::VisitAsyncCallbackReference
                .call_with_descriptor(
                    get_call_id(callbacks_name, reference).as_deref(),
                    out_path,
                    &(callbacks_name, reference, &extensions),
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_callback(
                        parsed_spec,
                        out_path,
                        None,
                        callback,
                        extensions,
                        call_stack,
                    )
                })
        }
        ReferenceOr::Item(callback) => Script::VisitAsyncCallbackStart
            .call_with_descriptor(
                callbacks_name,
                out_path,
                &(callbacks_name, callback, &extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                let mut current_call_stack = call_stack.clone();
                for it in callback {
                    current_call_stack = visit_path_item(
                        parsed_spec,
                        out_path,
                        Some(it.0),
                        it.1,
                        &current_call_stack,
                    )?;
                }
                Ok(current_call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitAsyncCallbackEnd.call_with_descriptor(
                    callbacks_name,
                    out_path,
                    &(callbacks_name, callback, &extensions),
                    call_stack,
                )
            }),
    }
}

pub fn visit_media_type(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    media_type_name: &str,
    media_type: &MediaType,
    call_stack: &CallStack,
) -> Result<CallStack> {
    Script::VisitMediaTypeStart
        .call_with_descriptor(
            Some(media_type_name),
            out_path,
            &(&media_type_name, media_type, &media_type.extensions),
            call_stack,
        )?
        .and_then(|call_stack| {
            if let Some(schema_ref) = &media_type.schema {
                visit_schema(parsed_spec, out_path, None, schema_ref, call_stack)
            } else {
                Ok(call_stack.clone())
            }
        })?
        .and_then(|call_stack| {
            visit_generic_example(
                out_path,
                &media_type.example,
                &media_type.extensions,
                call_stack,
            )
        })?
        .and_then(|call_stack| {
            visit_examples(
                parsed_spec,
                out_path,
                &media_type.examples,
                &media_type.extensions,
                call_stack,
            )
        })?
        .and_then(|call_stack| {
            visit_media_type_encodings(
                parsed_spec,
                out_path,
                &media_type.encoding,
                &media_type.extensions,
                call_stack,
            )
        })?
        .and_then(|call_stack| {
            Script::VisitMediaTypeEnd.call_with_descriptor(
                Some(media_type_name),
                out_path,
                &(&media_type_name, media_type, &media_type.extensions),
                call_stack,
            )
        })
}

pub fn visit_examples(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    examples: &IndexMap<String, ReferenceOr<Example>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if !examples.is_empty() {
        Script::VisitExamplesStart
            .call_with_descriptor(None, out_path, &(&examples, extensions), call_stack)?
            .and_then(|call_stack| {
                let mut current_call_stack = call_stack.clone();
                for it in examples {
                    current_call_stack = visit_example(
                        parsed_spec,
                        out_path,
                        Some(it.0),
                        it.1,
                        &current_call_stack,
                    )?;
                }
                Ok(current_call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitExamplesEnd.call_with_descriptor(
                    None,
                    out_path,
                    &(&examples, extensions),
                    call_stack,
                )
            })
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_request_bodies(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    request_bodies: &IndexMap<String, ReferenceOr<RequestBody>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if !request_bodies.is_empty() {
        Script::VisitRequestBodiesStart
            .call_with_descriptor(None, out_path, &(request_bodies, extensions), call_stack)?
            .and_then(|call_stack| {
                let mut current_call_stack = call_stack.clone();
                for it in request_bodies {
                    current_call_stack = visit_request_body(
                        parsed_spec,
                        out_path,
                        Some(it.0),
                        it.1,
                        &current_call_stack,
                    )?;
                }
                Ok(current_call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitRequestBodiesEnd.call_with_descriptor(
                    None,
                    out_path,
                    &(request_bodies, extensions),
                    call_stack,
                )
            })
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_generic_parameters(
    out_path: &Path,
    parameters: &IndexMap<String, serde_json::Value>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if !parameters.is_empty() {
        Script::VisitGenericParametersStart
            .call_with_descriptor(None, out_path, &(&parameters, extensions), call_stack)?
            .and_then(|call_stack| {
                let mut current_call_stack = call_stack.clone();
                for it in parameters {
                    current_call_stack = visit_generic_parameter(
                        out_path,
                        it.0,
                        it.1,
                        extensions,
                        &current_call_stack,
                    )?;
                }
                Ok(current_call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitGenericParametersEnd.call_with_descriptor(
                    None,
                    out_path,
                    &(&parameters, extensions),
                    call_stack,
                )
            })
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_header(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    header_name: Option<&str>,
    header: &ReferenceOr<Header>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    match header {
        ReferenceOr::Reference { reference } => {
            let header = references::resolve_reference::<Header>(reference, parsed_spec)?;
            Script::VisitHeaderReference
                .call_with_descriptor(
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
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_header(parsed_spec, out_path, None, header, call_stack)
                })
        }
        ReferenceOr::Item(header) => Script::VisitHeaderStart
            .call_with_descriptor(
                header_name,
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
                )
            })?
            .and_then(|call_stack| {
                visit_generic_example(out_path, &header.example, &header.extensions, call_stack)
            })?
            .and_then(|call_stack| {
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
                    header_name,
                    out_path,
                    &(&header_name, &header, &header.extensions),
                    call_stack,
                )
            }),
    }
}

pub fn visit_security_scheme(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    scheme_name: Option<&str>,
    security_scheme: &ReferenceOr<SecurityScheme>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    match security_scheme {
        ReferenceOr::Reference { reference } => {
            let scheme = references::resolve_reference::<SecurityScheme>(reference, parsed_spec)?;
            Script::VisitSecuritySchemeReference
                .call_with_descriptor(
                    get_call_id(scheme_name, reference).as_deref(),
                    out_path,
                    &(
                        &scheme_name,
                        reference,
                        match &scheme.as_item().expect(
                            "Unable to get extensions from resolved security scheme reference",
                        ) {
                            SecurityScheme::APIKey { extensions, .. } => extensions,
                            SecurityScheme::HTTP { extensions, .. } => extensions,
                            SecurityScheme::OAuth2 { extensions, .. } => extensions,
                            SecurityScheme::OpenIDConnect { extensions, .. } => extensions,
                        },
                    ),
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_security_scheme(parsed_spec, out_path, None, scheme, call_stack)
                })
        }
        ReferenceOr::Item(security_scheme) => match security_scheme {
            SecurityScheme::APIKey { .. } => {
                visit_security_scheme_apikey(out_path, scheme_name, security_scheme, call_stack)
            }
            SecurityScheme::HTTP { .. } => {
                visit_security_scheme_http(out_path, scheme_name, security_scheme, call_stack)
            }
            SecurityScheme::OAuth2 { .. } => {
                visit_security_scheme_oauth2(out_path, scheme_name, security_scheme, call_stack)
            }
            SecurityScheme::OpenIDConnect { .. } => visit_security_scheme_openid_connect(
                out_path,
                scheme_name,
                security_scheme,
                call_stack,
            ),
        },
    }
}

pub fn visit_headers(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    headers: &IndexMap<String, ReferenceOr<Header>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if !headers.is_empty() {
        Script::VisitHeadersStart
            .call_with_descriptor(None, out_path, &(headers, extensions), call_stack)?
            .and_then(|call_stack| {
                let mut current_call_stack = call_stack.clone();
                for it in headers {
                    current_call_stack =
                        visit_header(parsed_spec, out_path, Some(it.0), it.1, &current_call_stack)?;
                }
                Ok(current_call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitHeadersEnd.call_with_descriptor(
                    None,
                    out_path,
                    &(headers, extensions),
                    call_stack,
                )
            })
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_security_schemes(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    security_schemes: &IndexMap<String, ReferenceOr<SecurityScheme>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if !security_schemes.is_empty() {
        Script::VisitSecuritySchemesStart
            .call_with_descriptor(None, out_path, &(&security_schemes, extensions), call_stack)?
            .and_then(|call_stack| {
                let mut current_call_stack = call_stack.clone();
                for it in security_schemes {
                    current_call_stack = visit_security_scheme(
                        parsed_spec,
                        out_path,
                        Some(it.0),
                        it.1,
                        &current_call_stack,
                    )?;
                }
                Ok(current_call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitSecuritySchemesEnd.call_with_descriptor(
                    None,
                    out_path,
                    &(&security_schemes, extensions),
                    call_stack,
                )
            })
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_spec_tags(
    out_path: &Path,
    tags: &Vec<Tag>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if !tags.is_empty() {
        Script::VisitSpecTagsStart
            .call_with_descriptor(None, out_path, &(tags, extensions), call_stack)?
            .and_then(|call_stack| {
                let mut current_call_stack = call_stack.clone();
                for tag in tags {
                    current_call_stack =
                        visit_external_docs(out_path, &tag.external_docs, &current_call_stack)?;
                    current_call_stack = Script::VisitSpecTag.call_with_descriptor(
                        Some(&tag.name),
                        out_path,
                        &(tag, &tag.extensions),
                        &current_call_stack,
                    )?;
                }
                Ok(current_call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitSpecTagsEnd.call_with_descriptor(
                    None,
                    out_path,
                    &(tags, extensions),
                    call_stack,
                )
            })
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_security_requirements(
    out_path: &Path,
    securities: &Option<Vec<SecurityRequirement>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let Some(it) = securities.as_ref() {
        if !it.is_empty() {
            Script::VisitSecurityRequirementsStart
                .call_with_descriptor(None, out_path, &(it, extensions), call_stack)?
                .and_then(|call_stack| {
                    let mut current_call_stack = call_stack.clone();
                    for sec_map in it {
                        current_call_stack = Script::VisitSecurityRequirement
                            .call_with_descriptor(
                                None,
                                out_path,
                                &(sec_map, extensions),
                                &current_call_stack,
                            )?;
                    }
                    Ok(current_call_stack)
                })?
                .and_then(|call_stack| {
                    Script::VisitSecurityRequirementsEnd.call_with_descriptor(
                        None,
                        out_path,
                        &(it, extensions),
                        call_stack,
                    )
                })
        } else {
            Ok(call_stack.clone())
        }
    } else {
        Ok(call_stack.clone())
    }
}
pub fn visit_external_docs(
    out_path: &Path,
    external_docs: &Option<ExternalDocumentation>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let Some(it) = external_docs {
        Script::VisitExternalDocs.call_with_descriptor(
            Some(&it.url),
            out_path,
            &(it, &it.extensions),
            call_stack,
        )
    } else {
        Ok(call_stack.clone())
    }
}
pub fn visit_schemas(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    schemas: &IndexMap<String, ReferenceOr<Schema>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if !schemas.is_empty() {
        Script::VisitSchemasStart
            .call_with_descriptor(None, out_path, &(schemas, extensions), call_stack)?
            .and_then(|call_stack| {
                let mut current_call_stack = call_stack.clone();
                for (schema_name, schema_ref) in schemas {
                    current_call_stack = visit_schema(
                        parsed_spec,
                        out_path,
                        Some(schema_name),
                        schema_ref,
                        &current_call_stack,
                    )?;
                }
                Ok(current_call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitSchemasEnd.call_with_descriptor(
                    None,
                    out_path,
                    &(schemas, extensions),
                    call_stack,
                )
            })
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_responses(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    responses: &IndexMap<String, ReferenceOr<Response>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if !responses.is_empty() {
        Script::VisitResponsesStart
            .call_with_descriptor(None, out_path, &(responses, extensions), call_stack)?
            .and_then(|call_stack| {
                let mut current_call_stack = call_stack.clone();
                for (response_name, response_ref) in responses {
                    current_call_stack = visit_response(
                        parsed_spec,
                        out_path,
                        Some(response_name),
                        response_ref,
                        &current_call_stack,
                    )?;
                }
                Ok(current_call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitResponsesEnd.call_with_descriptor(
                    None,
                    out_path,
                    &(responses, extensions),
                    call_stack,
                )
            })
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_operation_responses(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    responses: &Responses,
    call_stack: &CallStack,
) -> Result<CallStack> {
    Script::VisitOperationResponsesStart
        .call_with_descriptor(
            None,
            out_path,
            &(&responses, &responses.extensions),
            call_stack,
        )?
        .and_then(|call_stack| {
            if let Some(response) = &responses.default {
                visit_response(parsed_spec, out_path, None, response, call_stack)
            } else {
                Ok(call_stack.clone())
            }
        })?
        .and_then(|call_stack| {
            let converted: IndexMap<String, ReferenceOr<Response>> = responses
                .responses
                .iter()
                .map(|(key, value)| (key.to_string(), value.clone()))
                .collect();

            visit_responses(
                parsed_spec,
                out_path,
                &converted,
                &responses.extensions,
                call_stack,
            )
        })?
        .and_then(|call_stack| {
            Script::VisitOperationResponsesEnd.call_with_descriptor(
                None,
                out_path,
                &(&responses, &responses.extensions),
                call_stack,
            )
        })
}

pub fn visit_parameters(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameters: &IndexMap<String, ReferenceOr<Parameter>>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if !parameters.is_empty() {
        Script::VisitParametersStart
            .call_with_descriptor(None, out_path, &(parameters, &extensions), call_stack)?
            .and_then(|call_stack| {
                let mut current_call_stack = call_stack.clone();
                for (parameter_name, parameter_ref) in parameters {
                    current_call_stack = visit_parameter(
                        parsed_spec,
                        out_path,
                        Some(parameter_name),
                        parameter_ref,
                        extensions,
                        &current_call_stack,
                    )?;
                }
                Ok(current_call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitParametersEnd.call_with_descriptor(
                    None,
                    out_path,
                    &(parameters, &extensions),
                    call_stack,
                )
            })
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_paths(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    paths: &Paths,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if !paths.paths.is_empty() {
        Script::VisitPathsStart
            .call_with_descriptor(None, out_path, &(&paths, &paths.extensions), call_stack)?
            .and_then(|call_stack| {
                let mut current_call_stack = call_stack.clone();
                for it in &paths.paths {
                    current_call_stack = visit_path_item_ref(
                        parsed_spec,
                        out_path,
                        Some(it.0),
                        it.1,
                        &current_call_stack,
                    )?;
                }
                Ok(current_call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitPathsEnd.call_with_descriptor(
                    None,
                    out_path,
                    &(&paths, &paths.extensions),
                    call_stack,
                )
            })
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_path_item_ref(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    path_item_name: Option<&str>,
    path_item_ref: &ReferenceOr<PathItem>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    match path_item_ref {
        ReferenceOr::Reference { reference } => {
            let path_item = references::resolve_reference::<PathItem>(reference, parsed_spec)?;
            Script::VisitPathItemReference
                .call_with_descriptor(
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
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_path_item_ref(parsed_spec, out_path, None, path_item, call_stack)
                })
        }
        ReferenceOr::Item(path_item) => {
            visit_path_item(parsed_spec, out_path, path_item_name, path_item, call_stack)
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
    call_stack: &CallStack,
) -> Result<CallStack> {
    match parameter_ref {
        ReferenceOr::Reference { reference } => {
            let parameter = references::resolve_reference::<Parameter>(reference, parsed_spec)?;
            Script::VisitParameterReference
                .call_with_descriptor(
                    get_call_id(parameter_name, reference).as_deref(),
                    out_path,
                    &(&parameter_name, reference, extensions),
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_parameter(
                        parsed_spec,
                        out_path,
                        None,
                        parameter,
                        extensions,
                        call_stack,
                    )
                })
        }
        ReferenceOr::Item(parameter) => match parameter {
            Parameter::Query { .. } => visit_query_parameter(
                parsed_spec,
                out_path,
                parameter_name,
                parameter,
                extensions,
                call_stack,
            ),
            Parameter::Header { .. } => visit_header_parameter(
                parsed_spec,
                out_path,
                parameter_name,
                parameter,
                extensions,
                call_stack,
            ),
            Parameter::Path { .. } => visit_path_parameter(
                parsed_spec,
                out_path,
                parameter_name,
                parameter,
                extensions,
                call_stack,
            ),
            Parameter::Cookie { .. } => visit_cookie_parameter(
                parsed_spec,
                out_path,
                parameter_name,
                parameter,
                extensions,
                call_stack,
            ),
        },
    }
}

pub fn visit_security_scheme_http(
    out_path: &Path,
    scheme_name: Option<&str>,
    http: &SecurityScheme,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let SecurityScheme::HTTP { extensions, .. } = http {
        Script::VisitSecuritySchemeHttp.call_with_descriptor(
            scheme_name,
            out_path,
            &(&scheme_name, &http, &extensions),
            call_stack,
        )
    } else {
        Err(anyhow!("Not a SecurityScheme.HTTP"))
    }
}

pub fn visit_security_scheme_oauth2(
    out_path: &Path,
    scheme_name: Option<&str>,
    oauth2: &SecurityScheme,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let SecurityScheme::OAuth2 {
        flows, extensions, ..
    } = oauth2
    {
        Script::VisitSecuritySchemeOAuth2Start
            .call_with_descriptor(
                scheme_name,
                out_path,
                &(&scheme_name, &oauth2, extensions),
                call_stack,
            )?
            .and_then(|call_stack| visit_security_scheme_oauth2_flows(out_path, flows, call_stack))?
            .and_then(|call_stack| {
                Script::VisitSecuritySchemeOAuth2End.call_with_descriptor(
                    scheme_name,
                    out_path,
                    &(&scheme_name, &oauth2, extensions),
                    call_stack,
                )
            })
    } else {
        Err(anyhow!("Not a SecurityScheme.OAuth2"))
    }
}

pub fn visit_security_scheme_oauth2_flows(
    out_path: &Path,
    flows: &OAuth2Flows,
    call_stack: &CallStack,
) -> Result<CallStack> {
    Script::VisitSecuritySchemeOAuth2FlowsStart
        .call_with_descriptor(None, out_path, &(flows, &flows.extensions), call_stack)?
        .and_then(|call_stack| {
            visit_security_scheme_oauth2_flows_implicit(out_path, &flows.implicit, call_stack)
        })?
        .and_then(|call_stack| {
            visit_security_scheme_oauth2_flows_password(out_path, &flows.password, call_stack)
        })?
        .and_then(|call_stack| {
            visit_security_scheme_oauth2_flows_client_credentials(
                out_path,
                &flows.client_credentials,
                call_stack,
            )
        })?
        .and_then(|call_stack| {
            visit_security_scheme_oauth2_flows_authorization_code(
                out_path,
                &flows.authorization_code,
                call_stack,
            )
        })?
        .and_then(|call_stack| {
            Script::VisitSecuritySchemeOAuth2FlowsEnd.call_with_descriptor(
                None,
                out_path,
                &(flows, &flows.extensions),
                call_stack,
            )
        })
}

pub fn visit_security_scheme_oauth2_flows_implicit(
    out_path: &Path,
    flow: &Option<ImplicitOAuth2Flow>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let Some(flow) = flow {
        Script::VisitSecuritySchemeOAuth2FlowImplicit.call_with_descriptor(
            None,
            out_path,
            &(flow, &flow.extensions),
            call_stack,
        )
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_security_scheme_oauth2_flows_password(
    out_path: &Path,
    flow: &Option<PasswordOAuth2Flow>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let Some(flow) = flow {
        Script::VisitSecuritySchemeOAuth2FlowPassword.call_with_descriptor(
            None,
            out_path,
            &(flow, &flow.extensions),
            call_stack,
        )
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_security_scheme_oauth2_flows_client_credentials(
    out_path: &Path,
    flow: &Option<ClientCredentialsOAuth2Flow>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let Some(flow) = flow {
        Script::VisitSecuritySchemeOAuth2FlowClientCredentials.call_with_descriptor(
            None,
            out_path,
            &(flow, &flow.extensions),
            call_stack,
        )
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_security_scheme_oauth2_flows_authorization_code(
    out_path: &Path,
    flow: &Option<AuthorizationCodeOAuth2Flow>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let Some(flow) = flow {
        Script::VisitSecuritySchemeOAuth2FlowAuthorizationCode.call_with_descriptor(
            None,
            out_path,
            &(flow, &flow.extensions),
            call_stack,
        )
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_security_scheme_apikey(
    out_path: &Path,
    scheme_name: Option<&str>,
    api_key: &SecurityScheme,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let SecurityScheme::APIKey { extensions, .. } = api_key {
        Script::VisitSecuritySchemeApiKey.call_with_descriptor(
            scheme_name,
            out_path,
            &(scheme_name, &api_key, &extensions),
            call_stack,
        )
    } else {
        Err(anyhow!("Not a SecurityScheme.APIKey"))
    }
}

pub fn visit_security_scheme_openid_connect(
    out_path: &Path,
    scheme_name: Option<&str>,
    openid_connect: &SecurityScheme,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let SecurityScheme::OpenIDConnect { extensions, .. } = openid_connect {
        Script::VisitSecuritySchemeOpenIdConnect.call_with_descriptor(
            scheme_name,
            out_path,
            &(scheme_name, &openid_connect, &extensions),
            call_stack,
        )
    } else {
        Err(anyhow!("Not a SecurityScheme.OpenIDConnect"))
    }
}

pub fn visit_parameter_data(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameter_data: &ParameterData,
    call_stack: &CallStack,
) -> Result<CallStack> {
    Script::VisitParameterDataStart
        .call_with_descriptor(
            Some(&parameter_data.name),
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
            )
        })?
        .and_then(|call_stack| {
            visit_generic_example(
                out_path,
                &parameter_data.example,
                &parameter_data.extensions,
                call_stack,
            )
        })?
        .and_then(|call_stack| {
            visit_examples(
                parsed_spec,
                out_path,
                &parameter_data.examples,
                &parameter_data.extensions,
                call_stack,
            )
        })?
        .and_then(|call_stack| {
            Script::VisitParameterDataEnd.call_with_descriptor(
                Some(&parameter_data.name),
                out_path,
                &(parameter_data, &parameter_data.extensions),
                call_stack,
            )
        })
}

pub fn visit_query_parameter(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameter_name: Option<&str>,
    parameter: &Parameter,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let Parameter::Query { parameter_data, .. } = parameter {
        Script::VisitQueryParameterStart
            .call_with_descriptor(
                parameter_name,
                out_path,
                &(parameter_name, &parameter, &extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_parameter_data(parsed_spec, out_path, parameter_data, call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitQueryParameterEnd.call_with_descriptor(
                    parameter_name,
                    out_path,
                    &(parameter_name, &parameter, &extensions),
                    call_stack,
                )
            })
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
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let Parameter::Header { parameter_data, .. } = parameter {
        Script::VisitHeaderParameterStart
            .call_with_descriptor(
                parameter_name,
                out_path,
                &(parameter_name, &parameter, &extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_parameter_data(parsed_spec, out_path, parameter_data, call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitHeaderParameterEnd.call_with_descriptor(
                    parameter_name,
                    out_path,
                    &(parameter_name, &parameter, &extensions),
                    call_stack,
                )
            })
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
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let Parameter::Path { parameter_data, .. } = parameter {
        Script::VisitPathParameterStart
            .call_with_descriptor(
                parameter_name,
                out_path,
                &(parameter_name, &parameter, &extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_parameter_data(parsed_spec, out_path, parameter_data, call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitPathParameterEnd.call_with_descriptor(
                    parameter_name,
                    out_path,
                    &(parameter_name, &parameter, &extensions),
                    call_stack,
                )
            })
    } else {
        Err(anyhow!("Not a Path parameter"))
    }
}

pub fn visit_path_item(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    path_item_name: Option<&str>,
    path_item: &PathItem,
    call_stack: &CallStack,
) -> Result<CallStack> {
    Script::VisitPathItemStart
        .call_with_descriptor(
            path_item_name,
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
            )
        })?
        .and_then(|call_stack| {
            visit_operation(
                parsed_spec,
                out_path,
                &path_item.put,
                &BracketScripts {
                    start: Script::VisitPutOperationStart,
                    end: Script::VisitPutOperationEnd,
                },
                call_stack,
            )
        })?
        .and_then(|call_stack| {
            visit_operation(
                parsed_spec,
                out_path,
                &path_item.post,
                &BracketScripts {
                    start: Script::VisitPostOperationStart,
                    end: Script::VisitPostOperationEnd,
                },
                call_stack,
            )
        })?
        .and_then(|call_stack| {
            visit_operation(
                parsed_spec,
                out_path,
                &path_item.patch,
                &BracketScripts {
                    start: Script::VisitPatchOperationStart,
                    end: Script::VisitPatchOperationEnd,
                },
                call_stack,
            )
        })?
        .and_then(|call_stack| {
            visit_operation(
                parsed_spec,
                out_path,
                &path_item.options,
                &BracketScripts {
                    start: Script::VisitOptionsOperationStart,
                    end: Script::VisitOptionsOperationEnd,
                },
                call_stack,
            )
        })?
        .and_then(|call_stack| {
            visit_operation(
                parsed_spec,
                out_path,
                &path_item.head,
                &BracketScripts {
                    start: Script::VisitHeadOperationStart,
                    end: Script::VisitHeadOperationEnd,
                },
                call_stack,
            )
        })?
        .and_then(|call_stack| {
            visit_operation(
                parsed_spec,
                out_path,
                &path_item.get,
                &BracketScripts {
                    start: Script::VisitGetOperationStart,
                    end: Script::VisitGetOperationEnd,
                },
                call_stack,
            )
        })?
        .and_then(|call_stack| {
            visit_operation(
                parsed_spec,
                out_path,
                &path_item.delete,
                &BracketScripts {
                    start: Script::VisitDeleteOperationStart,
                    end: Script::VisitDeleteOperationEnd,
                },
                call_stack,
            )
        })?
        .and_then(|call_stack| {
            visit_servers(
                out_path,
                &path_item.servers,
                &path_item.extensions,
                call_stack,
            )
        })?
        .and_then(|call_stack| {
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
            )
        })?
        .and_then(|call_stack| {
            Script::VisitPathItemEnd.call_with_descriptor(
                path_item_name,
                out_path,
                &(path_item_name, &path_item, &path_item.extensions),
                call_stack,
            )
        })
}

pub fn visit_operation(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    operation: &Option<Operation>,
    braced_scripts: &BracketScripts,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let Some(operation) = operation {
        braced_scripts
            .start
            .call_with_descriptor(
                operation
                    .operation_id
                    .clone()
                    .or_else(|| operation.summary.clone())
                    .as_deref(),
                out_path,
                &(&operation, &operation.extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_external_docs(out_path, &operation.external_docs, call_stack)
            })?
            .and_then(|call_stack| {
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
                )
            })?
            .and_then(|call_stack| {
                if let Some(request_body) = &operation.request_body {
                    visit_request_body(parsed_spec, out_path, None, request_body, call_stack)
                } else {
                    Ok(call_stack.clone())
                }
            })?
            .and_then(|call_stack| {
                visit_operation_responses(parsed_spec, out_path, &operation.responses, call_stack)
            })?
            .and_then(|call_stack| {
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
                )
            })?
            .and_then(|call_stack| {
                visit_security_requirements(
                    out_path,
                    &operation.security,
                    &operation.extensions,
                    call_stack,
                )
            })?
            .and_then(|call_stack| {
                visit_servers(
                    out_path,
                    &operation.servers,
                    &operation.extensions,
                    call_stack,
                )
            })?
            .and_then(|call_stack| {
                braced_scripts.end.call_with_descriptor(
                    operation
                        .operation_id
                        .clone()
                        .or_else(|| operation.summary.clone())
                        .as_deref(),
                    out_path,
                    &(&operation, &operation.extensions),
                    call_stack,
                )
            })
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_cookie_parameter(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    parameter_name: Option<&str>,
    parameter: &Parameter,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let Parameter::Cookie { parameter_data, .. } = parameter {
        Script::VisitCookieParameterStart
            .call_with_descriptor(
                parameter_name,
                out_path,
                &(parameter_name, &parameter, &extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_parameter_data(parsed_spec, out_path, parameter_data, call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitCookieParameterEnd.call_with_descriptor(
                    parameter_name,
                    out_path,
                    &(parameter_name, &parameter, &extensions),
                    call_stack,
                )
            })
    } else {
        Err(anyhow!("Not a Cookie parameter"))
    }
}

pub fn visit_spec_components(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    components: &Option<Components>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let Some(components) = components {
        Script::VisitComponentsStart
            .call_with_descriptor(
                None,
                out_path,
                &(components, &components.extensions),
                call_stack,
            )?
            .and_then(|call_stack| {
                visit_schemas(
                    parsed_spec,
                    out_path,
                    &components.schemas,
                    &components.extensions,
                    call_stack,
                )
            })?
            .and_then(|call_stack| {
                visit_responses(
                    parsed_spec,
                    out_path,
                    &components.responses,
                    &components.extensions,
                    call_stack,
                )
            })?
            .and_then(|call_stack| {
                visit_parameters(
                    parsed_spec,
                    out_path,
                    &components.parameters,
                    &components.extensions,
                    call_stack,
                )
            })?
            .and_then(|call_stack| {
                visit_examples(
                    parsed_spec,
                    out_path,
                    &components.examples,
                    &components.extensions,
                    call_stack,
                )
            })?
            .and_then(|call_stack| {
                visit_request_bodies(
                    parsed_spec,
                    out_path,
                    &components.request_bodies,
                    &components.extensions,
                    call_stack,
                )
            })?
            .and_then(|call_stack| {
                visit_headers(
                    parsed_spec,
                    out_path,
                    &components.headers,
                    &components.extensions,
                    call_stack,
                )
            })?
            .and_then(|call_stack| {
                visit_security_schemes(
                    parsed_spec,
                    out_path,
                    &components.security_schemes,
                    &components.extensions,
                    call_stack,
                )
            })?
            .and_then(|call_stack| {
                visit_links(
                    parsed_spec,
                    out_path,
                    &components.links,
                    &components.extensions,
                    call_stack,
                )
            })?
            .and_then(|call_stack| {
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
                    None,
                    out_path,
                    &(&components.extensions),
                    call_stack,
                )
            })
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_schema_default(
    out_path: &Path,
    default: &Option<serde_json::Value>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let Some(default) = default.as_ref() {
        Script::VisitDefault.call_with_descriptor(
            None,
            out_path,
            &(default, extensions),
            call_stack,
        )
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_spec_info(out_path: &Path, info: &Info, call_stack: &CallStack) -> Result<CallStack> {
    Script::VisitSpecInfoStart
        .call_with_descriptor(None, out_path, &(info, &info.extensions), call_stack)?
        .and_then(|call_stack| visit_spec_info_contact(out_path, &info.contact, call_stack))?
        .and_then(|call_stack| visit_spec_info_license(out_path, &info.license, call_stack))?
        .and_then(|call_stack| {
            Script::VisitSpecInfoEnd.call_with_descriptor(
                None,
                out_path,
                &(info, &info.extensions),
                call_stack,
            )
        })
}

pub fn visit_spec_info_contact(
    out_path: &Path,
    contact: &Option<Contact>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let Some(it) = contact {
        Script::VisitSpecInfoContact.call_with_descriptor(
            it.name.as_deref(),
            out_path,
            &(&it, &it.extensions),
            call_stack,
        )
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_spec_info_license(
    out_path: &Path,
    license: &Option<License>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if let Some(it) = license {
        Script::VisitSpecInfoLicense.call_with_descriptor(
            Some(&it.name),
            out_path,
            &(&it, &it.extensions),
            call_stack,
        )
    } else {
        Ok(call_stack.clone())
    }
}
pub fn visit_server(out_path: &Path, server: &Server, call_stack: &CallStack) -> Result<CallStack> {
    Script::VisitServerStart
        .call_with_descriptor(
            Some(&server.url),
            out_path,
            &(server, &server.extensions),
            call_stack,
        )?
        .and_then(|call_stack| {
            if let Some(variables) = server.variables.as_ref() {
                let mut current_call_stack = call_stack.clone();
                for it in variables {
                    current_call_stack = Script::VisitServerVariable.call_with_descriptor(
                        Some(it.0),
                        out_path,
                        &(&server.url, &it.0, &it.1, &it.1.extensions),
                        &current_call_stack,
                    )?;
                }
                Ok(current_call_stack)
            } else {
                Ok(call_stack.clone())
            }
        })?
        .and_then(|call_stack| {
            Script::VisitServerEnd.call_with_descriptor(
                Some(&server.url),
                out_path,
                &(server, &server.extensions),
                call_stack,
            )
        })
}

pub fn visit_servers(
    out_path: &Path,
    servers: &Vec<Server>,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    if !servers.is_empty() {
        Script::VisitServersStart
            .call_with_descriptor(None, out_path, &(servers, extensions), call_stack)?
            .and_then(|call_stack| {
                let mut current_call_stack = call_stack.clone();
                for server in servers {
                    current_call_stack = visit_server(out_path, server, &current_call_stack)?;
                }
                Ok(current_call_stack)
            })?
            .and_then(|call_stack| {
                Script::VisitServersEnd.call_with_descriptor(
                    None,
                    out_path,
                    &(servers, extensions),
                    call_stack,
                )
            })
    } else {
        Ok(call_stack.clone())
    }
}

pub fn visit_object_property(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    property_name: Option<&str>,
    property_schema_ref: &ReferenceOr<Schema>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    match property_schema_ref {
        ReferenceOr::Reference { reference } => {
            let property_schema = references::resolve_reference::<Schema>(reference, parsed_spec)?;
            Script::VisitObjectPropertyReference
                .call_with_descriptor(
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
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_object_property(parsed_spec, out_path, None, property_schema, call_stack)
                })
        }
        ReferenceOr::Item(schema) => {
            let schema = schema.as_schema();

            Script::VisitObjectPropertyStart
                .call_with_descriptor(
                    property_name,
                    out_path,
                    &(property_name, schema, &schema.schema_data.extensions),
                    call_stack,
                )?
                .and_then(|call_stack| {
                    visit_schema(parsed_spec, out_path, None, property_schema_ref, call_stack)
                })?
                .and_then(|call_stack| {
                    Script::VisitObjectPropertyEnd.call_with_descriptor(
                        property_name,
                        out_path,
                        &(property_name, schema, &schema.schema_data.extensions),
                        call_stack,
                    )
                })
        }
    }
}

pub fn visit_object(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    object_description: &ObjectType,
    extensions: &IndexMap<String, serde_json::Value>,
    call_stack: &CallStack,
) -> Result<CallStack> {
    Script::VisitObjectStart
        .call_with_descriptor(
            None,
            out_path,
            &(object_description, extensions),
            call_stack,
        )?
        .and_then(|call_stack| {
            let current_call_stack = if !object_description.properties.is_empty() {
                Script::VisitObjectPropertiesStart
                    .call_with_descriptor(
                        None,
                        out_path,
                        &(&object_description.properties, extensions),
                        call_stack,
                    )?
                    .and_then(|call_stack| {
                        let mut current_call_stack = call_stack.clone();
                        for (local_property_name, property_schema_ref) in
                            &object_description.properties
                        {
                            let unboxed = property_schema_ref.clone().unbox();
                            current_call_stack = visit_object_property(
                                parsed_spec,
                                out_path,
                                Some(local_property_name),
                                &unboxed,
                                &current_call_stack,
                            )?;
                        }
                        Ok(current_call_stack)
                    })?
                    .and_then(|call_stack| {
                        Script::VisitObjectPropertiesEnd.call_with_descriptor(
                            None,
                            out_path,
                            &(&object_description.properties, extensions),
                            call_stack,
                        )
                    })?
            } else {
                call_stack.clone()
            };

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
                            &current_call_stack,
                        )
                    }
                    openapiv3::AdditionalProperties::Schema(it) => {
                        let schema_ref = it.as_ref();
                        Script::VisitAdditionalPropertiesStart
                            .call_with_descriptor(
                                None,
                                out_path,
                                &(
                                    schema_ref,
                                    object_description.min_properties,
                                    object_description.max_properties,
                                    extensions,
                                ),
                                &current_call_stack,
                            )?
                            .and_then(|call_stack| {
                                visit_schema(parsed_spec, out_path, None, schema_ref, call_stack)
                            })?
                            .and_then(|call_stack| {
                                Script::VisitAdditionalPropertiesEnd.call_with_descriptor(
                                    None,
                                    out_path,
                                    &(
                                        schema_ref,
                                        object_description.min_properties,
                                        object_description.max_properties,
                                        extensions,
                                    ),
                                    call_stack,
                                )
                            })
                    }
                }
            } else {
                Ok(current_call_stack.clone())
            }
        })?
        .and_then(|call_stack| {
            Script::VisitObjectEnd.call_with_descriptor(
                None,
                out_path,
                &(object_description, extensions),
                call_stack,
            )
        })
}
