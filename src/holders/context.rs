use std::{
    any::Any,
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};

use clap::Parser;
use indexmap::IndexMap;
use log::debug;
use mlua::Lua;
use once_cell::sync::Lazy;
use reqwest::blocking::Client;

use crate::Cli;
use anyhow::{anyhow, Result};

pub const DEFAULT_TESTS_OPENAPI_DIR_NAME: &str = "openapi";
pub const DEFAULT_TESTS_OUT_DIR_NAME: &str = "actual";
pub const DEFAULT_TESTS_EXPECTED_DIR_NAME: &str = "expected";
pub const DEFAULT_TESTS_OPENAPI_FILE_NAME: &str = "openapi.yml";
pub const DEFAULT_LOGS_COLOR_MODE: &str = "always";
pub const DEFAULT_LOGS_LOG_LEVEL: &str = "debug";

pub const EXTENSION_FOR_NAME: &str = "x-ot-name";
pub const EXTENSION_ANY_ADDITIONAL_PROPERTIES_NAME: &str = "x-ot-additional-properties-name";
pub const EXTENSION_TARGET_PARAMETERS_NAME: &str = "x-ot-target-parameters";
pub const TARGET_PARAMETERS_NAME_IN_LUA: &str = "targetParameters";

///Scripts names
pub const DEFAULT_OBJECT_ADDITIONAL_PROPERTIES: &str = "additionalProperties";

//schemas
pub const SCRIPT_PRELUDE: &str = "prelude";
pub const SCRIPT_RESPONSES_START: &str = "visitors/components/responses/responses_start";
pub const SCRIPT_RESPONSES_END: &str = "visitors/components/responses/responses_end";
pub const SCRIPT_RESPONSE_START: &str = "visitors/components/responses/response_start";
pub const SCRIPT_RESPONSE_END: &str = "visitors/components/responses/response_end";
pub const SCRIPT_RESPONSE_HEADERS_START: &str =
    "visitors/components/responses/headers/response_headers_start";
pub const SCRIPT_RESPONSE_HEADERS_END: &str =
    "visitors/components/responses/headers/response_headers_end";
pub const SCRIPT_RESPONSE_HEADER_START: &str =
    "visitors/components/responses/headers/header/response_header_start";
pub const SCRIPT_RESPONSE_HEADER_END: &str =
    "visitors/components/responses/headers/header/response_header_end";
pub const SCRIPT_RESPONSE_HEADER_EXAMPLE: &str =
    "visitors/components/responses/headers/header/response_header_example";
pub const SCRIPT_RESPONSE_HEADER_EXAMPLES_EXAMPLE: &str =
    "visitors/components/responses/headers/header/response_header_examples_example";
pub const SCRIPT_RESPONSE_HEADER_EXAMPLES_START: &str =
    "visitors/components/responses/headers/header/response_header_examples_start";
pub const SCRIPT_RESPONSE_HEADER_EXAMPLES_END: &str =
    "visitors/components/responses/headers/header/response_header_examples_end";
pub const SCRIPT_RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_START: &str =
    "visitors/components/responses/headers/header/format/response_header_examples_example_format_start";
pub const SCRIPT_RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_END: &str =
    "visitors/components/responses/headers/header/format/response_header_examples_example_format_end";
pub const SCRIPT_RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_START: &str =
    "visitors/components/responses/headers/header/format/response_header_examples_example_format_media_type_start";
pub const SCRIPT_RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_END: &str =
    "visitors/components/responses/headers/header/format/response_header_examples_example_format_media_type_end";
pub const SCRIPT_RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_EXAMPLE: &str =
    "visitors/components/responses/headers/header/format/response_header_examples_example_format_media_type_example";
pub const SCRIPT_RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_ENCODING_START: &str =
    "visitors/components/responses/headers/header/format/response_header_examples_example_format_media_type_encoding_start";
pub const SCRIPT_RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_ENCODING_END: &str =
    "visitors/components/responses/headers/header/format/response_header_examples_example_format_media_type_encoding_end";

pub const SCRIPT_SCHEMAS_START: &str = "visitors/components/schemas/schemas_start";
pub const SCRIPT_SCHEMAS_END: &str = "visitors/components/schemas/schemas_end";
pub const SCRIPT_SCHEMA_START: &str = "visitors/components/schemas/schema_start";

pub const SCRIPT_SCHEMA_END: &str = "visitors/components/schemas/schema_end";
pub const SCRIPT_SCHEMA_EXTERNAL_DOCS: &str = "visitors/components/schemas/external_docs";
pub const SCRIPT_SCHEMA_EXAMPLE: &str = "visitors/components/schemas/example";
pub const SCRIPT_SCHEMA_DEFAULT: &str = "visitors/components/schemas/default";
pub const SCRIPT_SCHEMA_DISCRIMINATOR: &str = "visitors/components/schemas/discriminator";

pub const SCRIPT_SPEC_EXTERNAL_DOCS: &str = "visitors/spec_external_docs";
pub const SCRIPT_SPEC_START: &str = "visitors/spec_start";
pub const SCRIPT_SPEC_END: &str = "visitors/spec_end";

pub const SCRIPT_SPEC_TAG_EXTERNAL_DOCS: &str = "visitors/tags/spec_tag_external_docs";
pub const SCRIPT_SPEC_TAG: &str = "visitors/tags/spec_tag";
pub const SCRIPT_SPEC_TAGS_END: &str = "visitors/tags/spec_tags_end";
pub const SCRIPT_SPEC_TAGS_START: &str = "visitors/tags/spec_tags_start";

pub const SCRIPT_SPEC_SERVERS_START: &str = "visitors/servers/spec_servers_start";
pub const SCRIPT_SPEC_SERVER: &str = "visitors/servers/spec_server";
pub const SCRIPT_SPEC_SERVER_VARIABLE: &str = "visitors/servers/spec_server_variable";
pub const SCRIPT_SPEC_SERVERS_END: &str = "visitors/servers/spec_servers_end";

pub const SCRIPT_SPEC_INFO: &str = "visitors/info/spec_info";
pub const SCRIPT_SPEC_INFO_CONTACT: &str = "visitors/info/spec_info_contact";
pub const SCRIPT_SPEC_INFO_LICENSE: &str = "visitors/info/spec_info_license";

pub const SCRIPT_SPEC_SECURITIES_START: &str = "visitors/securities/spec_securities_start";
pub const SCRIPT_SPEC_SECURITY: &str = "visitors/securities/spec_security";
pub const SCRIPT_SPEC_SECURITIES_END: &str = "visitors/securities/spec_securities_end";

pub const SCRIPT_OBJECT_START: &str = "visitors/components/schemas/kind/type/object/object_start";
pub const SCRIPT_OBJECT_END: &str = "visitors/components/schemas/kind/type/object/object_end";
pub const SCRIPT_ANY_SCHEMA: &str = "visitors/components/schemas/kind/any/any_schema";
//properties
pub const SCRIPT_NOT_PROPERTY_START: &str =
    "visitors/components/schemas/kind/not/not_property_start";

pub const SCRIPT_NOT_PROPERTY_END: &str = "visitors/components/schemas/kind/not/not_property_end";
pub const SCRIPT_OBJECT_ADDITIONAL_PROPERTIES: &str =
    "visitors/components/schemas/kind/type/object/object_additional_properties_any";
pub const SCRIPT_STRING_PROPERTY: &str =
    "visitors/components/schemas/kind/type/string/string_property";
pub const SCRIPT_NUMBER_PROPERTY: &str =
    "visitors/components/schemas/kind/type/number/number_property";
pub const SCRIPT_INTEGER_PROPERTY: &str =
    "visitors/components/schemas/kind/type/integer/integer_property";
pub const SCRIPT_ARRAY_PROPERTY_START: &str =
    "visitors/components/schemas/kind/type/object/array_property_start";
pub const SCRIPT_ARRAY_PROPERTY_END: &str =
    "visitors/components/schemas/kind/type/object/array_property_end";
pub const SCRIPT_BOOLEAN_PROPERTY: &str =
    "visitors/components/schemas/kind/type/boolean/boolean_property";
//group_of
pub const SCRIPT_ONE_OF_START: &str = "visitors/components/schemas/kind/oneOf/one_of_start";
pub const SCRIPT_ONE_OF_END: &str = "visitors/components/schemas/kind/oneOf/one_of_end";
pub const SCRIPT_ALL_OF_START: &str = "visitors/components/schemas/kind/allOf/all_of_start";
pub const SCRIPT_ALL_OF_END: &str = "visitors/components/schemas/kind/allOf/all_of_end";
pub const SCRIPT_ANY_OF_START: &str = "visitors/components/schemas/kind/anyOf/any_of_start";
pub const SCRIPT_ANY_OF_END: &str = "visitors/components/schemas/kind/anyOf/any_of_end";
pub static CLIENT: Lazy<Client> = Lazy::new(reqwest::blocking::Client::new);
pub static CLI: Lazy<&'static Cli> = Lazy::new(|| Box::leak(Box::new(Cli::parse())));
pub static LUA_VM: Lazy<Mutex<Lua>> = Lazy::new(|| Mutex::new(Lua::new()));
pub static LOG_CONTEXT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
pub static EMPTY_INDEXMAP: Lazy<IndexMap<String, serde_json::Value>> = Lazy::new(IndexMap::new);

pub fn get_lua_vm() -> MutexGuard<'static, Lua> {
    LUA_VM.lock().expect("Could not lock lua vm")
}

pub fn recreate_lua_vm() {
    let mut lua_vm = LUA_VM.lock().expect("Could not lock lua vm");
    *lua_vm = Lua::new();
}

type Cache = HashMap<String, &'static (dyn Any + Send + Sync)>;
pub static CACHE: Lazy<Mutex<Cache>> = Lazy::new(|| Mutex::new(Cache::new()));

pub fn compute_if_absent<F, R>(key: String, factory: F) -> Result<&'static R>
where
    R: 'static + Any + Send + Sync,
    F: FnOnce() -> Result<R>,
{
    let read_cache = CACHE.lock().expect("Could not lock cache");

    let cached = match read_cache.get(&key) {
        Some(value) => {
            debug!("Used cached value [{}]", key);
            *value
        }
        None => {
            drop(read_cache);
            let leaked_value: &'static R = Box::leak(Box::new(factory()?));
            let mut write_cache = CACHE.lock().expect("Could not lock cache");
            write_cache.insert(key, leaked_value);
            leaked_value
        }
    };

    cached
        .downcast_ref::<R>()
        .ok_or_else(|| anyhow!("Could not cast value to out cached type"))
}
