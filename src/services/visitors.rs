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
    AnySchema, ArrayType, Components, Contact, ExternalDocumentation, Header, Info, IntegerType,
    License, NumberType, ObjectType, OpenAPI, ReferenceOr, Response, Schema, SchemaData,
    SecurityRequirement, Server, StringType, Tag,
};
use serde::de::DeserializeOwned;

use crate::{
    holders::context::{
        CACHE, DEFAULT_EXTENSION_ANY_ADDITIONAL_PROPERTIES_NAME, DEFAULT_EXTENSION_FOR_NAME,
        DEFAULT_EXTENSION_FOR_NOT_PROPERTY_NAME, DEFAULT_OBJECT_ADDITIONAL_PROPERTIES,
        SCRIPT_ALL_OF_END, SCRIPT_ALL_OF_SCHEMA_END, SCRIPT_ALL_OF_SCHEMA_START,
        SCRIPT_ALL_OF_START, SCRIPT_ANY_OF_END, SCRIPT_ANY_OF_SCHEMA_END,
        SCRIPT_ANY_OF_SCHEMA_START, SCRIPT_ANY_OF_START, SCRIPT_ANY_SCHEMA,
        SCRIPT_ARRAY_PROPERTY_END, SCRIPT_ARRAY_PROPERTY_START, SCRIPT_BOOLEAN_PROPERTY,
        SCRIPT_INTEGER_PROPERTY, SCRIPT_NOT_PROPERTY_END, SCRIPT_NOT_PROPERTY_START,
        SCRIPT_NUMBER_PROPERTY, SCRIPT_OBJECT_ADDITIONAL_PROPERTIES,
        SCRIPT_OBJECT_ADDITIONAL_PROPERTIES_END, SCRIPT_OBJECT_ADDITIONAL_PROPERTIES_START,
        SCRIPT_OBJECT_END, SCRIPT_OBJECT_PROPERTY_END, SCRIPT_OBJECT_PROPERTY_START,
        SCRIPT_OBJECT_START, SCRIPT_ONE_OF_END, SCRIPT_ONE_OF_SCHEMA_END,
        SCRIPT_ONE_OF_SCHEMA_START, SCRIPT_ONE_OF_START, SCRIPT_RESPONSES_END,
        SCRIPT_RESPONSES_START, SCRIPT_RESPONSE_END, SCRIPT_RESPONSE_HEADER_END,
        SCRIPT_RESPONSE_HEADER_EXAMPLE, SCRIPT_RESPONSE_HEADER_START, SCRIPT_RESPONSE_START,
        SCRIPT_SCHEMAS_END, SCRIPT_SCHEMAS_START, SCRIPT_SCHEMA_DEFAULT,
        SCRIPT_SCHEMA_DISCRIMINATOR, SCRIPT_SCHEMA_END, SCRIPT_SCHEMA_EXAMPLE,
        SCRIPT_SCHEMA_EXTERNAL_DOCS, SCRIPT_SCHEMA_START, SCRIPT_SPEC_END,
        SCRIPT_SPEC_EXTERNAL_DOCS, SCRIPT_SPEC_INFO, SCRIPT_SPEC_INFO_CONTACT,
        SCRIPT_SPEC_INFO_LICENSE, SCRIPT_SPEC_SECURITIES_END, SCRIPT_SPEC_SECURITIES_START,
        SCRIPT_SPEC_SECURITY, SCRIPT_SPEC_SERVER, SCRIPT_SPEC_SERVERS_END,
        SCRIPT_SPEC_SERVERS_START, SCRIPT_SPEC_SERVER_VARIABLE, SCRIPT_SPEC_START, SCRIPT_SPEC_TAG,
        SCRIPT_SPEC_TAGS_END, SCRIPT_SPEC_TAGS_START, SCRIPT_SPEC_TAG_EXTERNAL_DOCS,
        SCRIPT_STRING_PROPERTY,
    },
    services::{references, schema, scripts},
    structs::common::{ModelName, ParsedSpec},
    traits::common::AsSchemaRef,
    Commands,
};
use anyhow::Result;
use anyhow::{anyhow, Context};

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

        cli::set_global_lua_parameters(&openapi)?;

        visit_spec_start(out_path, &openapi)?;
        visit_spec_info(out_path, &openapi.info)?;
        visit_spec_servers(out_path, &openapi.servers, &openapi.extensions)?;
        visit_spec_security(out_path, &openapi.security, &openapi.extensions)?;
        visit_spec_tags(out_path, &openapi.tags, &openapi.extensions)?;
        visit_spec_external_docs(out_path, &openapi.external_docs)?;
        visit_spec_components(
            out_path,
            spec_path,
            Arc::new(spec_as_json),
            &openapi.components,
        )?;
        visit_spec_end(out_path, &openapi)?;

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
    names_stack: Vec<ModelName>,
    schema_ref: &ReferenceOr<T>,
) -> Result<()>
where
    T: DeserializeOwned + Send + Sync + AsSchemaRef + From<Schema> + 'static,
{
    let schema_extensions = schema::get_extensions_with_schema_resolving(parsed_spec, schema_ref)?;

    let mut property_stack = names_stack.clone();
    property_stack.push(ModelName {
        base: String::from("not"),
        extended: schema_extensions
            .get(DEFAULT_EXTENSION_FOR_NOT_PROPERTY_NAME)
            .cloned(),
    });

    scripts::call_with_descriptor(
        out_path,
        &(&property_stack, schema_extensions),
        SCRIPT_NOT_PROPERTY_START,
    )?;

    visit_schema(parsed_spec, out_path, property_stack.clone(), schema_ref)?;

    scripts::call_with_descriptor(
        out_path,
        &(property_stack, schema_extensions),
        SCRIPT_NOT_PROPERTY_END,
    )
}

pub fn visit_schema<T>(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    mut names_stack: Vec<ModelName>,
    schema_ref: &ReferenceOr<T>,
) -> Result<()>
where
    T: DeserializeOwned + Send + Sync + AsSchemaRef + From<Schema> + 'static,
{
    match schema_ref {
        ReferenceOr::Reference { reference } => {
            visit_schema(
                parsed_spec,
                out_path,
                names_stack.clone(),
                references::resolve_reference::<T>(reference, parsed_spec)?,
            )?;
        }
        ReferenceOr::Item(schema_item) => {
            let schema_extensions = &schema_item.as_schema().schema_data.extensions;

            if let Some(it) = names_stack.last_mut() {
                it.extended = schema_extensions.get(DEFAULT_EXTENSION_FOR_NAME).cloned();
            }

            let schema_data = &schema_item.as_schema().schema_data;

            scripts::call_with_descriptor(
                out_path,
                &(names_stack.clone(), schema_data, schema_extensions),
                SCRIPT_SCHEMA_START,
            )?;

            visit_discriminator(out_path, names_stack.clone(), schema_data)?;
            visit_schema_external_docs(out_path, names_stack.clone(), schema_data)?;
            visit_schema_example(out_path, names_stack.clone(), schema_data)?;
            visit_schema_default(out_path, names_stack.clone(), schema_data)?;

            match &schema_item.as_schema().schema_kind {
                openapiv3::SchemaKind::Type(type_) => match type_ {
                    openapiv3::Type::Object(object_descriptor) => visit_object(
                        parsed_spec,
                        out_path,
                        names_stack.clone(),
                        object_descriptor,
                        schema_extensions,
                    )?,
                    openapiv3::Type::Array(array_descriptor) => visit_array(
                        parsed_spec,
                        out_path,
                        names_stack.clone(),
                        array_descriptor,
                        schema_extensions,
                    )?,
                    // Simple types
                    openapiv3::Type::String(string_descriptor) => visit_string(
                        out_path,
                        names_stack.clone(),
                        string_descriptor,
                        schema_extensions,
                    )?,
                    openapiv3::Type::Number(number_descriptor) => visit_number(
                        out_path,
                        names_stack.clone(),
                        number_descriptor,
                        schema_extensions,
                    )?,
                    openapiv3::Type::Integer(integer_descriptor) => visit_integer(
                        out_path,
                        names_stack.clone(),
                        integer_descriptor,
                        schema_extensions,
                    )?,
                    openapiv3::Type::Boolean(_) => {
                        visit_boolean(out_path, names_stack.clone(), schema_extensions)?
                    }
                },
                openapiv3::SchemaKind::OneOf { one_of } => visit_one_of(
                    parsed_spec,
                    out_path,
                    names_stack.clone(),
                    one_of,
                    schema_extensions,
                )?,
                openapiv3::SchemaKind::AllOf { all_of } => visit_all_of(
                    parsed_spec,
                    out_path,
                    names_stack.clone(),
                    all_of,
                    schema_extensions,
                )?,
                openapiv3::SchemaKind::AnyOf { any_of } => visit_any_of(
                    parsed_spec,
                    out_path,
                    names_stack.clone(),
                    any_of,
                    schema_extensions,
                )?,
                openapiv3::SchemaKind::Not { not } => {
                    visit_not(parsed_spec, out_path, names_stack.clone(), not)?
                }
                openapiv3::SchemaKind::Any(any_schema) => {
                    visit_any_schema(out_path, names_stack.clone(), any_schema, schema_extensions)?
                }
            }
            scripts::call_with_descriptor(
                out_path,
                &(names_stack, schema_data, schema_extensions),
                SCRIPT_SCHEMA_END,
            )?;
        }
    }
    Ok(())
}

pub fn visit_response(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    mut names_stack: Vec<ModelName>,
    response_ref: &ReferenceOr<Response>,
) -> Result<()> {
    match response_ref {
        ReferenceOr::Reference { reference } => {
            visit_response(
                parsed_spec,
                out_path,
                names_stack.clone(),
                references::resolve_reference::<Response>(reference, parsed_spec)?,
            )?;
        }
        ReferenceOr::Item(it) => {
            let response_extensions = &it.extensions;

            if let Some(it) = names_stack.last_mut() {
                it.extended = response_extensions.get(DEFAULT_EXTENSION_FOR_NAME).cloned();
            }

            scripts::call_with_descriptor(
                out_path,
                &(names_stack.clone(), it, response_extensions),
                SCRIPT_RESPONSE_START,
            )?;

            visit_headers(
                parsed_spec,
                out_path,
                names_stack.clone(),
                &it.headers,
                response_extensions,
            )?;

            scripts::call_with_descriptor(
                out_path,
                &(names_stack, it, response_extensions),
                SCRIPT_RESPONSE_END,
            )?;
        }
    }
    Ok(())
}

pub fn visit_string(
    out_path: &Path,
    names_stack: Vec<ModelName>,
    string_descriptor: &StringType,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    scripts::call_with_descriptor(
        out_path,
        &(names_stack, string_descriptor, extensions),
        SCRIPT_STRING_PROPERTY,
    )
}

pub fn visit_any_schema(
    out_path: &Path,
    names_stack: Vec<ModelName>,
    any_schema_descriptor: &AnySchema,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    scripts::call_with_descriptor(
        out_path,
        &(names_stack, any_schema_descriptor, extensions),
        SCRIPT_ANY_SCHEMA,
    )
}

pub fn visit_number(
    out_path: &Path,
    names_stack: Vec<ModelName>,
    number_descriptor: &NumberType,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    scripts::call_with_descriptor(
        out_path,
        &(names_stack, number_descriptor, extensions),
        SCRIPT_NUMBER_PROPERTY,
    )
}

pub fn visit_integer(
    out_path: &Path,
    names_stack: Vec<ModelName>,
    integer_descriptor: &IntegerType,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    scripts::call_with_descriptor(
        out_path,
        &(names_stack, integer_descriptor, extensions),
        SCRIPT_INTEGER_PROPERTY,
    )
}

pub fn visit_array(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: Vec<ModelName>,
    array_descriptor: &ArrayType,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    scripts::call_with_descriptor(
        out_path,
        &(names_stack.clone(), array_descriptor, extensions),
        SCRIPT_ARRAY_PROPERTY_START,
    )?;

    array_descriptor
        .items
        .as_ref()
        .map(|schema_ref| -> Result<()> {
            let array_item_extensions =
                schema::get_extensions_with_schema_resolving(parsed_spec, schema_ref)?;
            let mut property_name_stack = names_stack.clone();
            // Array it is as object with one property with name items
            property_name_stack.push(ModelName {
                base: String::from("items"),
                extended: array_item_extensions
                    .get(DEFAULT_EXTENSION_FOR_NAME)
                    .cloned(),
            });
            visit_schema(parsed_spec, out_path, property_name_stack, schema_ref)?;
            Ok(())
        })
        .transpose()?;

    scripts::call_with_descriptor(
        out_path,
        &(names_stack, array_descriptor, extensions),
        SCRIPT_ARRAY_PROPERTY_END,
    )
}

pub fn visit_boolean(
    out_path: &Path,
    names_stack: Vec<ModelName>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    scripts::call_with_descriptor(
        out_path,
        &(names_stack, extensions),
        SCRIPT_BOOLEAN_PROPERTY,
    )
}

pub fn visit_one_of(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: Vec<ModelName>,
    schemas: &[ReferenceOr<Schema>],
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    scripts::call_with_descriptor(out_path, &(&names_stack, extensions), SCRIPT_ONE_OF_START)?;

    schemas.iter().enumerate().try_for_each(|it| {
        let schema_extensions = schema::get_extensions_with_schema_resolving(parsed_spec, it.1)?;

        let mut current_schema_stack = names_stack.clone();
        current_schema_stack.push(ModelName {
            base: format!("oneOf-{}", it.0),
            extended: schema_extensions.get(DEFAULT_EXTENSION_FOR_NAME).cloned(),
        });

        scripts::call_with_descriptor(
            out_path,
            &(&current_schema_stack, schema_extensions),
            SCRIPT_ONE_OF_SCHEMA_START,
        )?;

        visit_schema(parsed_spec, out_path, current_schema_stack.clone(), it.1)?;

        scripts::call_with_descriptor(
            out_path,
            &(&current_schema_stack, schema_extensions),
            SCRIPT_ONE_OF_SCHEMA_END,
        )
    })?;

    scripts::call_with_descriptor(out_path, &(names_stack, extensions), SCRIPT_ONE_OF_END)
}

pub fn visit_all_of(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: Vec<ModelName>,
    schemas: &[ReferenceOr<Schema>],
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    scripts::call_with_descriptor(out_path, &(&names_stack, extensions), SCRIPT_ALL_OF_START)?;

    schemas.iter().enumerate().try_for_each(|it| {
        let schema_extensions = schema::get_extensions_with_schema_resolving(parsed_spec, it.1)?;

        let mut current_schema_stack = names_stack.clone();
        current_schema_stack.push(ModelName {
            base: format!("allOf-{}", it.0),
            extended: schema_extensions.get(DEFAULT_EXTENSION_FOR_NAME).cloned(),
        });

        scripts::call_with_descriptor(
            out_path,
            &(&current_schema_stack, schema_extensions),
            SCRIPT_ALL_OF_SCHEMA_START,
        )?;

        visit_schema(parsed_spec, out_path, current_schema_stack.clone(), it.1)?;

        scripts::call_with_descriptor(
            out_path,
            &(&current_schema_stack, schema_extensions),
            SCRIPT_ALL_OF_SCHEMA_END,
        )
    })?;

    scripts::call_with_descriptor(out_path, &(names_stack, extensions), SCRIPT_ALL_OF_END)
}

pub fn visit_any_of(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: Vec<ModelName>,
    schemas: &[ReferenceOr<Schema>],
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    scripts::call_with_descriptor(out_path, &(&names_stack, extensions), SCRIPT_ANY_OF_START)?;

    schemas.iter().enumerate().try_for_each(|it| {
        let schema_extensions = schema::get_extensions_with_schema_resolving(parsed_spec, it.1)?;

        let mut current_schema_stack = names_stack.clone();
        current_schema_stack.push(ModelName {
            base: format!("anyOf-{}", it.0),
            extended: schema_extensions.get(DEFAULT_EXTENSION_FOR_NAME).cloned(),
        });

        scripts::call_with_descriptor(
            out_path,
            &(&current_schema_stack, schema_extensions),
            SCRIPT_ANY_OF_SCHEMA_START,
        )?;

        visit_schema(parsed_spec, out_path, current_schema_stack.clone(), it.1)?;

        scripts::call_with_descriptor(
            out_path,
            &(&current_schema_stack, schema_extensions),
            SCRIPT_ANY_OF_SCHEMA_END,
        )
    })?;

    scripts::call_with_descriptor(out_path, &(names_stack, extensions), SCRIPT_ANY_OF_END)
}

pub fn visit_discriminator(
    out_path: &Path,
    names_stack: Vec<ModelName>,
    schema_data: &SchemaData,
) -> Result<()> {
    if let Some(discriminator) = schema_data.discriminator.as_ref() {
        let mut property_stack = names_stack.clone();
        property_stack.push(ModelName {
            base: String::from("discriminator"),
            extended: discriminator
                .extensions
                .get(DEFAULT_EXTENSION_FOR_NAME)
                .cloned(),
        });

        scripts::call_with_descriptor(
            out_path,
            &(property_stack, discriminator, &discriminator.extensions),
            SCRIPT_SCHEMA_DISCRIMINATOR,
        )?;
    }
    Ok(())
}

pub fn visit_header(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: Vec<ModelName>,
    header_name: &str,
    header: &ReferenceOr<Header>,
) -> Result<()> {
    match header {
        ReferenceOr::Reference { reference } => {
            visit_header(
                parsed_spec,
                out_path,
                names_stack,
                header_name,
                references::resolve_reference::<Header>(reference, parsed_spec)?,
            )?;
        }
        ReferenceOr::Item(header) => {
            let mut property_stack = names_stack.clone();
            property_stack.push(ModelName {
                base: header_name.to_owned(),
                extended: header.extensions.get(DEFAULT_EXTENSION_FOR_NAME).cloned(),
            });

            scripts::call_with_descriptor(
                out_path,
                &(&property_stack, &header, &header.extensions),
                SCRIPT_RESPONSE_HEADER_START,
            )?;

            scripts::call_with_descriptor(
                out_path,
                &(&property_stack, &header.example, &header.extensions),
                SCRIPT_RESPONSE_HEADER_EXAMPLE,
            )?;

            // header.example

            scripts::call_with_descriptor(
                out_path,
                &(&property_stack, &header, &header.extensions),
                SCRIPT_RESPONSE_HEADER_END,
            )?;
        }
    }
    Ok(())
}

pub fn visit_headers(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: Vec<ModelName>,
    headers: &IndexMap<String, ReferenceOr<Header>>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    // scripts::call_with_descriptor(
    //     out_path,
    //     &(names_stack.clone(), headers, extensions),
    //     SCRIPT_RESPONSE_HEADERS_START,
    // )?;

    // headers
    //     .iter()
    //     .try_for_each(|it| visit_header(parsed_spec, out_path, names_stack.clone(), it.0, it.1))?;

    // scripts::call_with_descriptor(
    //     out_path,
    //     &(names_stack.clone(), headers, extensions),
    //     SCRIPT_RESPONSE_HEADERS_END,
    // )?;
    Ok(())
}

pub fn visit_schema_external_docs(
    out_path: &Path,
    names_stack: Vec<ModelName>,
    schema_data: &SchemaData,
) -> Result<()> {
    if let Some(external_docs) = schema_data.external_docs.as_ref() {
        let mut property_stack = names_stack.clone();
        property_stack.push(ModelName {
            base: String::from("externalDocs"),
            extended: external_docs
                .extensions
                .get(DEFAULT_EXTENSION_FOR_NAME)
                .cloned(),
        });

        scripts::call_with_descriptor(
            out_path,
            &(property_stack, external_docs, &external_docs.extensions),
            SCRIPT_SCHEMA_EXTERNAL_DOCS,
        )?;
    }
    Ok(())
}

pub fn visit_spec_tags(
    out_path: &Path,
    tags: &Vec<Tag>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    scripts::call_with_descriptor(out_path, &(tags, extensions), SCRIPT_SPEC_TAGS_START)?;

    tags.iter().try_for_each(|tag| {
        visit_spec_tag_external_docs(out_path, &tag.external_docs)?;
        scripts::call_with_descriptor(
            out_path,
            &(
                &tag.name,
                &tag.description,
                &tag.external_docs,
                &tag.extensions,
            ),
            SCRIPT_SPEC_TAG,
        )
    })?;

    scripts::call_with_descriptor(out_path, &(tags, extensions), SCRIPT_SPEC_TAGS_END)?;

    Ok(())
}

pub fn visit_spec_security(
    out_path: &Path,
    security: &Option<Vec<SecurityRequirement>>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    if let Some(it) = security.as_ref() {
        scripts::call_with_descriptor(out_path, &(it, extensions), SCRIPT_SPEC_SECURITIES_START)?;

        it.iter().try_for_each(|sec_map| {
            scripts::call_with_descriptor(out_path, &(sec_map, extensions), SCRIPT_SPEC_SECURITY)
        })?;

        scripts::call_with_descriptor(out_path, &(it, extensions), SCRIPT_SPEC_SECURITIES_END)?;
    }
    Ok(())
}

pub fn visit_spec_external_docs(
    out_path: &Path,
    external_docs: &Option<ExternalDocumentation>,
) -> Result<()> {
    if let Some(it) = external_docs {
        scripts::call_with_descriptor(out_path, &(it, &it.extensions), SCRIPT_SPEC_EXTERNAL_DOCS)?;
    }
    Ok(())
}

pub fn visit_spec_components(
    out_path: &Path,
    spec_path: &Path,
    spec_as_json: Arc<serde_json::Value>,
    components: &Option<Components>,
) -> Result<()> {
    if let Some(it) = components {
        let parsed_spec = ParsedSpec {
            path: spec_path.to_owned(),
            spec: spec_as_json,
        };

        scripts::call_with_descriptor(out_path, &(&it.extensions), SCRIPT_SCHEMAS_START)?;

        it.schemas
            .iter()
            .try_for_each(|(schema_name, schema_ref)| {
                visit_schema(
                    &parsed_spec,
                    out_path,
                    vec![ModelName::new(schema_name.to_owned())],
                    schema_ref,
                )
            })?;

        scripts::call_with_descriptor(out_path, &(&it.extensions), SCRIPT_SCHEMAS_END)?;

        scripts::call_with_descriptor(out_path, &(&it.extensions), SCRIPT_RESPONSES_START)?;

        it.responses
            .iter()
            .try_for_each(|(response_name, response_ref)| {
                visit_response(
                    &parsed_spec,
                    out_path,
                    vec![ModelName::new(response_name.to_owned())],
                    response_ref,
                )
            })?;

        scripts::call_with_descriptor(out_path, &(&it.extensions), SCRIPT_RESPONSES_END)?;
    }
    Ok(())
}

pub fn visit_spec_tag_external_docs(
    out_path: &Path,
    external_docs: &Option<ExternalDocumentation>,
) -> Result<()> {
    if let Some(it) = external_docs {
        scripts::call_with_descriptor(
            out_path,
            &(it, &it.extensions),
            SCRIPT_SPEC_TAG_EXTERNAL_DOCS,
        )?;
    }
    Ok(())
}

pub fn visit_schema_example(
    out_path: &Path,
    names_stack: Vec<ModelName>,
    schema_data: &SchemaData,
) -> Result<()> {
    if let Some(docs) = schema_data.example.as_ref() {
        let mut property_stack = names_stack.clone();
        property_stack.push(ModelName {
            base: String::from("example"),
            extended: None,
        });

        scripts::call_with_descriptor(
            out_path,
            &(property_stack, docs, &schema_data.extensions),
            SCRIPT_SCHEMA_EXAMPLE,
        )?;
    }
    Ok(())
}

pub fn visit_schema_default(
    out_path: &Path,
    names_stack: Vec<ModelName>,
    schema_data: &SchemaData,
) -> Result<()> {
    if let Some(docs) = schema_data.example.as_ref() {
        let mut property_stack = names_stack.clone();
        property_stack.push(ModelName {
            base: String::from("default"),
            extended: None,
        });

        scripts::call_with_descriptor(
            out_path,
            &(property_stack, docs, &schema_data.extensions),
            SCRIPT_SCHEMA_DEFAULT,
        )?;
    }
    Ok(())
}

pub fn visit_spec_start(out_path: &Path, openapi: &OpenAPI) -> Result<()> {
    scripts::call_with_descriptor(
        out_path,
        &(&openapi.openapi, &openapi.extensions),
        SCRIPT_SPEC_START,
    )?;
    Ok(())
}

pub fn visit_spec_end(out_path: &Path, openapi: &OpenAPI) -> Result<()> {
    scripts::call_with_descriptor(
        out_path,
        &(&openapi.openapi, &openapi.extensions),
        SCRIPT_SPEC_END,
    )?;
    Ok(())
}

pub fn visit_spec_info(out_path: &Path, info: &Info) -> Result<()> {
    visit_spec_info_contact(out_path, &info.contact)?;
    visit_spec_info_license(out_path, &info.license)?;

    scripts::call_with_descriptor(out_path, &(&info, &info.extensions), SCRIPT_SPEC_INFO)?;
    Ok(())
}

pub fn visit_spec_info_contact(out_path: &Path, contact: &Option<Contact>) -> Result<()> {
    if let Some(it) = contact {
        scripts::call_with_descriptor(out_path, &(&it, &it.extensions), SCRIPT_SPEC_INFO_CONTACT)?;
    }
    Ok(())
}

pub fn visit_spec_info_license(out_path: &Path, contact: &Option<License>) -> Result<()> {
    if let Some(it) = contact {
        scripts::call_with_descriptor(out_path, &(&it, &it.extensions), SCRIPT_SPEC_INFO_LICENSE)?;
    }
    Ok(())
}

pub fn visit_spec_servers(
    out_path: &Path,
    servers: &Vec<Server>,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    scripts::call_with_descriptor(out_path, &(servers, extensions), SCRIPT_SPEC_SERVERS_START)?;

    servers.iter().try_for_each(|server| {
        if let Some(variables) = server.variables.as_ref() {
            variables.iter().try_for_each(|it| {
                scripts::call_with_descriptor(
                    out_path,
                    &(
                        &it.0,
                        &it.1.enumeration,
                        &it.1.default,
                        &it.1.description,
                        &it.1.extensions,
                    ),
                    SCRIPT_SPEC_SERVER_VARIABLE,
                )
            })?;
        }
        scripts::call_with_descriptor(
            out_path,
            &(
                &server.url,
                &server.description,
                &server.variables,
                &server.extensions,
            ),
            SCRIPT_SPEC_SERVER,
        )
    })?;

    scripts::call_with_descriptor(out_path, &(servers, extensions), SCRIPT_SPEC_SERVERS_END)?;

    Ok(())
}

pub fn visit_object(
    parsed_spec: &ParsedSpec,
    out_path: &Path,
    names_stack: Vec<ModelName>,
    object_description: &ObjectType,
    extensions: &IndexMap<String, serde_json::Value>,
) -> Result<()> {
    scripts::call_with_descriptor(
        out_path,
        &(names_stack.clone(), object_description, extensions),
        SCRIPT_OBJECT_START,
    )?;

    object_description.properties.iter().try_for_each(
        |(local_property_name, property_schema_ref)| -> Result<()> {
            let property_extensions =
                schema::get_extensions_with_schema_resolving(parsed_spec, property_schema_ref)?;

            let mut property_stack = names_stack.clone();
            property_stack.push(ModelName {
                base: local_property_name.to_owned(),
                extended: property_extensions.get(DEFAULT_EXTENSION_FOR_NAME).cloned(),
            });

            scripts::call_with_descriptor(
                out_path,
                &(&property_stack, object_description, property_extensions),
                SCRIPT_OBJECT_PROPERTY_START,
            )?;

            visit_schema(
                parsed_spec,
                out_path,
                property_stack.clone(),
                property_schema_ref,
            )?;

            scripts::call_with_descriptor(
                out_path,
                &(&property_stack, object_description, property_extensions),
                SCRIPT_OBJECT_PROPERTY_END,
            )
        },
    )?;

    if let Some(it) = object_description.additional_properties.as_ref() {
        match it {
            openapiv3::AdditionalProperties::Any(value) => {
                let mut property_stack = names_stack.clone();
                //AdditionalProperties it is just especial one property
                property_stack.push(ModelName {
                    base: DEFAULT_OBJECT_ADDITIONAL_PROPERTIES.to_owned(),
                    // additionalProperties does not have schema, so extensions to it sent from object level
                    extended: extensions
                        .get(DEFAULT_EXTENSION_ANY_ADDITIONAL_PROPERTIES_NAME)
                        .cloned(),
                });

                scripts::call_with_descriptor(
                    out_path,
                    &(
                        &property_stack,
                        *value,
                        object_description.min_properties,
                        object_description.max_properties,
                        extensions,
                    ),
                    SCRIPT_OBJECT_ADDITIONAL_PROPERTIES,
                )?;
            }
            openapiv3::AdditionalProperties::Schema(it) => {
                let additional_properties_schema = it.as_ref();

                let additional_properties_extensions =
                    schema::get_extensions_with_schema_resolving(
                        parsed_spec,
                        additional_properties_schema,
                    )?;

                let mut property_stack = names_stack.clone();
                //AdditionalProperties it is just especial one property
                property_stack.push(ModelName {
                    base: DEFAULT_OBJECT_ADDITIONAL_PROPERTIES.to_owned(),
                    extended: additional_properties_extensions
                        .get(DEFAULT_EXTENSION_ANY_ADDITIONAL_PROPERTIES_NAME)
                        .cloned(),
                });

                scripts::call_with_descriptor(
                    out_path,
                    &(
                        &property_stack,
                        object_description.min_properties,
                        object_description.max_properties,
                        additional_properties_extensions,
                    ),
                    SCRIPT_OBJECT_ADDITIONAL_PROPERTIES_START,
                )?;

                visit_schema(
                    parsed_spec,
                    out_path,
                    property_stack.clone(),
                    additional_properties_schema,
                )?;

                scripts::call_with_descriptor(
                    out_path,
                    &(
                        &property_stack,
                        object_description.min_properties,
                        object_description.max_properties,
                        additional_properties_extensions,
                    ),
                    SCRIPT_OBJECT_ADDITIONAL_PROPERTIES_END,
                )?;
            }
        }
    }

    scripts::call_with_descriptor(
        out_path,
        &(names_stack.clone(), object_description, extensions),
        SCRIPT_OBJECT_END,
    )
}
