use indexmap::IndexMap;
use openapiv3::{ReferenceOr, Schema};
use serde::de::DeserializeOwned;

use crate::{structs::common::ParsedSpec, traits::common::AsSchemaRef};

use super::references;
use anyhow::Result;

pub fn get_extension_with_schema_resolving<T>(
    parsed_spec: &ParsedSpec,
    extension_name: &str,
    schema_ref: &ReferenceOr<T>,
) -> Result<Option<serde_json::Value>>
where
    T: DeserializeOwned + AsSchemaRef + From<Schema> + Send + Sync + 'static,
{
    match schema_ref {
        ReferenceOr::Reference { reference } => get_extension_with_schema_resolving(
            parsed_spec,
            extension_name,
            references::resolve_reference::<T>(reference, parsed_spec)?,
        ),
        ReferenceOr::Item(schema_item) => Ok(schema_item
            .as_schema()
            .schema_data
            .extensions
            .get(extension_name)
            .cloned()),
    }
}

pub fn get_extensions_with_schema_resolving<'a, T>(
    parsed_spec: &'a ParsedSpec,
    schema_ref: &'a ReferenceOr<T>,
) -> Result<&'a IndexMap<String, serde_json::Value>>
where
    T: DeserializeOwned + AsSchemaRef + From<Schema> + Send + Sync + 'static,
{
    match schema_ref {
        ReferenceOr::Reference { reference } => get_extensions_with_schema_resolving(
            parsed_spec,
            references::resolve_reference::<T>(reference, parsed_spec)?,
        ),
        ReferenceOr::Item(schema_item) => Ok(&schema_item.as_schema().schema_data.extensions),
    }
}
