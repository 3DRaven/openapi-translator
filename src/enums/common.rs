use std::{fmt::Display, ops::Add, path::Path};

use anyhow::{anyhow, Context, Result};
use mlua::LuaSerdeExt;
use serde::{Deserialize, Serialize};

use crate::{
    holders::context::get_lua_vm,
    services::{code, scripts},
    structs::common::CallStack,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WriteMode {
    #[serde(rename = "APPEND")]
    Append,
    #[serde(rename = "PREPEND")]
    Prepend,
    #[serde(rename = "REMOVE")]
    Remove,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Script {
    #[serde(rename = "PRELUDE")]
    Prelude,
    #[serde(rename = "RESPONSES_START")]
    ResponsesStart,
    #[serde(rename = "RESPONSES_END")]
    ResponsesEnd,
    #[serde(rename = "RESPONSE_START")]
    ResponseStart,
    #[serde(rename = "RESPONSE_END")]
    ResponseEnd,
    #[serde(rename = "RESPONSE_HEADERS_START")]
    ResponseHeadersStart,
    #[serde(rename = "RESPONSE_HEADERS_END")]
    ResponseHeadersEnd,
    #[serde(rename = "RESPONSE_HEADER_START")]
    ResponseHeaderStart,
    #[serde(rename = "RESPONSE_HEADER_END")]
    ResponseHeaderEnd,
    #[serde(rename = "RESPONSE_HEADER_EXAMPLE")]
    ResponseHeaderExample,
    #[serde(rename = "EXAMPLES_EXAMPLE")]
    ExamplesExample,
    #[serde(rename = "EXAMPLES_START")]
    ExamplesStart,
    #[serde(rename = "EXAMPLES_END")]
    ExamplesEnd,
    #[serde(rename = "PARAMETER_SCHEMA_OR_CONTENT_START")]
    ParameterSchemaOrContentStart,
    #[serde(rename = "PARAMETER_SCHEMA_OR_CONTENT_END")]
    ParameterSchemaOrContentEnd,
    #[serde(rename = "MEDIA_TYPE_START")]
    MediaTypeStart,
    #[serde(rename = "MEDIA_TYPE_END")]
    MediaTypeEnd,
    #[serde(rename = "MEDIA_TYPE_EXAMPLE")]
    MediaTypeExample,
    #[serde(rename = "ENCODING_START")]
    EncodingStart,
    #[serde(rename = "ENCODING_END")]
    EncodingEnd,

    #[serde(rename = "SCHEMAS_START")]
    SchemasStart,
    #[serde(rename = "SCHEMAS_END")]
    SchemasEnd,
    #[serde(rename = "SCHEMA_START")]
    SchemaStart,
    #[serde(rename = "SCHEMA_END")]
    SchemaEnd,
    #[serde(rename = "SCHEMA_EXTERNAL_DOCS")]
    SchemaExternalDocs,
    #[serde(rename = "SCHEMA_EXAMPLE")]
    SchemaExample,
    #[serde(rename = "SCHEMA_DEFAULT")]
    SchemaDefault,
    #[serde(rename = "SCHEMA_DISCRIMINATOR")]
    SchemaDiscriminator,

    #[serde(rename = "SPEC_START")]
    SpecStart,
    #[serde(rename = "SPEC_END")]
    SpecEnd,

    #[serde(rename = "EXTERNAL_DOCS")]
    ExternalDocs,
    #[serde(rename = "SPEC_TAG")]
    SpecTag,
    #[serde(rename = "SPEC_TAGS_END")]
    SpecTagsEnd,
    #[serde(rename = "SPEC_TAGS_START")]
    SpecTagsStart,

    #[serde(rename = "SPEC_SERVERS_START")]
    SpecServersStart,
    #[serde(rename = "SPEC_SERVER_START")]
    SpecServerStart,
    #[serde(rename = "SPEC_SERVER_END")]
    SpecServerEnd,
    #[serde(rename = "SPEC_SERVER_VARIABLE")]
    SpecServerVariable,
    #[serde(rename = "SPEC_SERVERS_END")]
    SpecServersEnd,

    #[serde(rename = "SPEC_INFO_START")]
    SpecInfoStart,
    #[serde(rename = "SPEC_INFO_END")]
    SpecInfoEnd,
    #[serde(rename = "SPEC_INFO_CONTACT")]
    SpecInfoContact,
    #[serde(rename = "SPEC_INFO_LICENSE")]
    SpecInfoLicense,

    #[serde(rename = "SPEC_SECURITIES_START")]
    SpecSecuritiesStart,
    #[serde(rename = "SPEC_SECURITY")]
    SpecSecurity,
    #[serde(rename = "SPEC_SECURITIES_END")]
    SpecSecuritiesEnd,

    #[serde(rename = "OBJECT_START")]
    ObjectStart,
    #[serde(rename = "OBJECT_PROPERTY_START")]
    ObjectPropertyStart,
    #[serde(rename = "OBJECT_PROPERTY_END")]
    ObjectPropertyEnd,
    #[serde(rename = "OBJECT_END")]
    ObjectEnd,
    #[serde(rename = "ANY_SCHEMA")]
    AnySchema,

    #[serde(rename = "NOT_PROPERTY_START")]
    NotPropertyStart,
    #[serde(rename = "NOT_PROPERTY_END")]
    NotPropertyEnd,
    #[serde(rename = "OBJECT_ADDITIONAL_PROPERTIES_ANY")]
    ObjectAdditionalPropertiesAny,
    #[serde(rename = "OBJECT_ADDITIONAL_PROPERTIES_START")]
    ObjectAdditionalPropertiesStart,
    #[serde(rename = "OBJECT_ADDITIONAL_PROPERTIES_END")]
    ObjectAdditionalPropertiesEnd,
    #[serde(rename = "STRING_PROPERTY")]
    StringProperty,
    #[serde(rename = "NUMBER_PROPERTY")]
    NumberProperty,
    #[serde(rename = "INTEGER_PROPERTY")]
    IntegerProperty,
    #[serde(rename = "ARRAY_PROPERTY_START")]
    ArrayPropertyStart,
    #[serde(rename = "ARRAY_PROPERTY_END")]
    ArrayPropertyEnd,
    #[serde(rename = "BOOLEAN_PROPERTY")]
    BooleanProperty,

    #[serde(rename = "ONE_OF_START")]
    OneOfStart,
    #[serde(rename = "ONE_OF_END")]
    OneOfEnd,
    #[serde(rename = "ALL_OF_START")]
    AllOfStart,
    #[serde(rename = "ALL_OF_END")]
    AllOfEnd,
    #[serde(rename = "ANY_OF_START")]
    AnyOfStart,
    #[serde(rename = "ANY_OF_END")]
    AnyOfEnd,
}

impl Script {
    pub fn call_with_descriptor<T>(
        &self,
        out_path: &Path,
        descriptor: &T,
        call_stack: &CallStack,
    ) -> Result<CallStack>
    where
        T: Serialize,
    {
        let lua_vm = get_lua_vm();
        let func = scripts::get_lua_function(self, &lua_vm)?;

        let args_value = serde_json::to_value(descriptor)?;

        let args_vec = if args_value.is_array() {
            args_value
                .as_array()
                .ok_or_else(|| anyhow!("Not a array of args in descriptor"))?
                .iter()
                .map(|it| lua_vm.to_value(it))
                .collect::<Result<Vec<_>, _>>()?
        } else {
            vec![lua_vm.to_value(&args_value)?]
        };

        let code: mlua::Value = match &args_vec {
            args if args.len() == 1 => func.call((&args[0], lua_vm.to_value(call_stack))),
            args if args.len() == 2 => func.call((&args[0], &args[1], lua_vm.to_value(call_stack))),
            args if args.len() == 3 => {
                func.call((&args[0], &args[1], &args[2], lua_vm.to_value(call_stack)))
            }
            args if args.len() == 4 => func.call((
                &args[0],
                &args[1],
                &args[2],
                &args[3],
                lua_vm.to_value(call_stack),
            )),
            args if args.len() == 5 => func.call((
                &args[0],
                &args[1],
                &args[2],
                &args[3],
                &args[4],
                lua_vm.to_value(call_stack),
            )),
            args if args.len() == 6 => func.call((
                &args[0],
                &args[1],
                &args[2],
                &args[3],
                &args[4],
                &args[5],
                lua_vm.to_value(call_stack),
            )),
            args if args.len() == 7 => func.call((
                &args[0],
                &args[1],
                &args[2],
                &args[3],
                &args[4],
                &args[5],
                &args[6],
                lua_vm.to_value(call_stack),
            )),
            args if args.len() == 8 => func.call((
                &args[0],
                &args[1],
                &args[2],
                &args[3],
                &args[4],
                &args[5],
                &args[6],
                &args[7],
                lua_vm.to_value(call_stack),
            )),
            args if args.len() == 9 => func.call((
                &args[0],
                &args[1],
                &args[2],
                &args[3],
                &args[4],
                &args[5],
                &args[6],
                &args[7],
                &args[8],
                lua_vm.to_value(call_stack),
            )),
            args if args.len() == 10 => func.call((
                &args[0],
                &args[1],
                &args[2],
                &args[3],
                &args[4],
                &args[5],
                &args[6],
                &args[7],
                &args[8],
                &args[9],
                lua_vm.to_value(call_stack),
            )),
            _ => {
                panic!(
                    "Unknown number of parameters for script [{}] with args [{:?}]",
                    self, args_vec
                )
            }
        }
        .with_context(|| {
            format!(
                "Failed to call lua script [{}] with args [{:?}]",
                self, args_vec
            )
        })?;

        code::save_code(out_path, lua_vm.from_value(code)?)?;

        Ok(call_stack + self)
    }

    pub fn call_func(&self) -> Result<CallStack> {
        let lua_vm = get_lua_vm();
        let func = scripts::get_lua_function(self, &lua_vm)?;
        func.call::<_, ()>(())
            .with_context(|| format!("Could not call lua function [{}]", self))?;
        Ok(CallStack::new(self.clone()))
    }
}

impl Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("Script enum serialization error")
        )
    }
}

impl From<&Script> for &'static str {
    fn from(script: &Script) -> &'static str {
        match script {
            Script::Prelude => "prelude",
            Script::ResponsesStart => "visitors/components/responses/responses_start",
            Script::ResponsesEnd => "visitors/components/responses/responses_end",
            Script::ResponseStart => "visitors/components/responses/response_start",
            Script::ResponseEnd => "visitors/components/responses/response_end",
            Script::ResponseHeadersStart => {
                "visitors/components/responses/headers/response_headers_start"
            }
            Script::ResponseHeadersEnd => {
                "visitors/components/responses/headers/response_headers_end"
            }
            Script::ResponseHeaderStart => {
                "visitors/components/responses/headers/header/response_header_start"
            }
            Script::ResponseHeaderEnd => {
                "visitors/components/responses/headers/header/response_header_end"
            }
            Script::ResponseHeaderExample => {
                "visitors/components/responses/headers/header/response_header_example"
            }
            Script::ExamplesExample => "visitors/common/examples/examples_example",
            Script::ExamplesStart => "visitors/common/examples/examples_start",
            Script::ExamplesEnd => "visitors/common/examples/examples_end",
            Script::ParameterSchemaOrContentStart => {
                "visitors/common/parameter_schema_or_content/parameter_schema_or_content_start"
            }
            Script::ParameterSchemaOrContentEnd => {
                "visitors/common/parameter_schema_or_content/parameter_schema_or_content_end"
            }
            Script::MediaTypeStart => "visitors/common/media_type/media_type_start",
            Script::MediaTypeEnd => "visitors/common/media_type/media_type_end",
            Script::MediaTypeExample => "visitors/common/media_type/media_type_example",
            Script::EncodingStart => "visitors/common/media_type/encoding/encoding_start",
            Script::EncodingEnd => "visitors/common/media_type/encoding/encoding_end",

            Script::SchemasStart => "visitors/components/schemas/schemas_start",
            Script::SchemasEnd => "visitors/components/schemas/schemas_end",
            Script::SchemaStart => "visitors/components/schemas/schema_start",
            Script::SchemaEnd => "visitors/components/schemas/schema_end",
            Script::SchemaExternalDocs => "visitors/components/schemas/external_docs",
            Script::SchemaExample => "visitors/components/schemas/example",
            Script::SchemaDefault => "visitors/components/schemas/default",
            Script::SchemaDiscriminator => "visitors/components/schemas/discriminator",

            Script::SpecStart => "visitors/spec_start",
            Script::SpecEnd => "visitors/spec_end",

            Script::ExternalDocs => "visitors/common/external_docs/external_docs",
            Script::SpecTag => "visitors/tags/spec_tag",
            Script::SpecTagsEnd => "visitors/tags/spec_tags_end",
            Script::SpecTagsStart => "visitors/tags/spec_tags_start",

            Script::SpecServersStart => "visitors/servers/spec_servers_start",
            Script::SpecServerStart => "visitors/servers/spec_server_start",
            Script::SpecServerEnd => "visitors/servers/spec_server_end",
            Script::SpecServerVariable => "visitors/servers/spec_server_variable",
            Script::SpecServersEnd => "visitors/servers/spec_servers_end",

            Script::SpecInfoStart => "visitors/info/spec_info_start",
            Script::SpecInfoEnd => "visitors/info/spec_info_end",

            Script::SpecInfoContact => "visitors/info/spec_info_contact",
            Script::SpecInfoLicense => "visitors/info/spec_info_license",

            Script::SpecSecuritiesStart => "visitors/securities/spec_securities_start",
            Script::SpecSecurity => "visitors/securities/spec_security",
            Script::SpecSecuritiesEnd => "visitors/securities/spec_securities_end",

            Script::ObjectStart => "visitors/components/schemas/kind/type/object/object_start",
            Script::ObjectEnd => "visitors/components/schemas/kind/type/object/object_end",
            Script::AnySchema => "visitors/components/schemas/kind/any/any_schema",

            Script::NotPropertyStart => "visitors/components/schemas/kind/not/not_property_start",
            Script::NotPropertyEnd => "visitors/components/schemas/kind/not/not_property_end",
            Script::ObjectAdditionalPropertiesAny => {
                "visitors/components/schemas/kind/type/object/object_additional_properties_any"
            }
            Script::StringProperty => {
                "visitors/components/schemas/kind/type/string/string_property"
            }
            Script::NumberProperty => {
                "visitors/components/schemas/kind/type/number/number_property"
            }
            Script::IntegerProperty => {
                "visitors/components/schemas/kind/type/integer/integer_property"
            }
            Script::ArrayPropertyStart => {
                "visitors/components/schemas/kind/type/object/array_property_start"
            }
            Script::ArrayPropertyEnd => {
                "visitors/components/schemas/kind/type/object/array_property_end"
            }
            Script::BooleanProperty => {
                "visitors/components/schemas/kind/type/boolean/boolean_property"
            }
            Script::OneOfStart => "visitors/components/schemas/kind/oneOf/one_of_start",
            Script::OneOfEnd => "visitors/components/schemas/kind/oneOf/one_of_end",
            Script::AllOfStart => "visitors/components/schemas/kind/allOf/all_of_start",
            Script::AllOfEnd => "visitors/components/schemas/kind/allOf/all_of_end",
            Script::AnyOfStart => "visitors/components/schemas/kind/anyOf/any_of_start",
            Script::AnyOfEnd => "visitors/components/schemas/kind/anyOf/any_of_end",
            Script::ObjectAdditionalPropertiesStart => {
                "visitors/components/schemas/kind/type/object/object_additional_properties_start"
            }
            Script::ObjectAdditionalPropertiesEnd => {
                "visitors/components/schemas/kind/type/object/object_additional_properties_end"
            }
            Script::ObjectPropertyStart => {
                "visitors/components/schemas/kind/type/object/object_property_start"
            }
            Script::ObjectPropertyEnd => {
                "visitors/components/schemas/kind/type/object/object_property_end"
            }
        }
    }
}

impl<'a> Add<Script> for &'a [Script] {
    type Output = Vec<Script>;

    fn add(self, rhs: Script) -> Self::Output {
        let mut new_stack = Vec::with_capacity(self.len() + 1);
        new_stack.extend_from_slice(self);
        new_stack.push(rhs);
        new_stack
    }
}
