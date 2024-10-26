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
    #[serde(rename = "OPERATION_RESPONSES_START")]
    OperationResponsesStart,
    #[serde(rename = "OPERATION_RESPONSES_END")]
    OperationResponsesEnd,
    #[serde(rename = "COMPONENTS_RESPONSES_START")]
    ComponentsResponsesStart,
    #[serde(rename = "COMPONENTS_RESPONSES_END")]
    ComponentsResponsesEnd,
    #[serde(rename = "PARAMETER_DATA_START")]
    ParameterDataStart,
    #[serde(rename = "PARAMETER_DATA_END")]
    ParameterDataEnd,
    #[serde(rename = "SECURITY_SCHEME_API_KEY")]
    SecuritySchemeApiKey,
    #[serde(rename = "SECURITY_SCHEME_OPENID_CONNECT")]
    SecuritySchemeOpenIdConnect,
    #[serde(rename = "SECURITY_SCHEME_OAUTH2_FLOW_IMPLICIT")]
    SecuritySchemeOAuth2FlowImplicit,
    #[serde(rename = "SECURITY_SCHEME_OAUTH2_FLOW_PASSWORD")]
    SecuritySchemeOAuth2FlowPassword,
    #[serde(rename = "SECURITY_SCHEME_OAUTH2_FLOW_CLIENT_CREDENTIALS")]
    SecuritySchemeOAuth2FlowClientCredentials,
    #[serde(rename = "SECURITY_SCHEME_OAUTH2_FLOW_AUTHORIZATION_CODE")]
    SecuritySchemeOAuth2FlowAuthorizationCode,
    #[serde(rename = "SECURITY_SCHEME_HTTP")]
    SecuritySchemeHttp,
    #[serde(rename = "SECURITY_SCHEME_OAUTH2_START")]
    SecuritySchemeOAuth2Start,
    #[serde(rename = "SECURITY_SCHEME_OAUTH2_END")]
    SecuritySchemeOAuth2End,
    #[serde(rename = "SECURITY_SCHEME_OAUTH2_FLOWS_START")]
    SecuritySchemeOAuth2FlowsStart,
    #[serde(rename = "SECURITY_SCHEME_OAUTH2_FLOWS_END")]
    SecuritySchemeOAuth2FlowsEnd,
    #[serde(rename = "QUERY_PARAMETER_START")]
    QueryParameterStart,
    #[serde(rename = "QUERY_PARAMETER_END")]
    QueryParameterEnd,
    #[serde(rename = "HEADER_PARAMETER_START")]
    HeaderParameterStart,
    #[serde(rename = "HEADER_PARAMETER_END")]
    HeaderParameterEnd,
    #[serde(rename = "PATH_PARAMETER_START")]
    PathParameterStart,
    #[serde(rename = "PATH_PARAMETER_END")]
    PathParameterEnd,
    #[serde(rename = "PATH_ITEM_START")]
    PathItemStart,
    #[serde(rename = "PATH_ITEM_END")]
    PathItemEnd,
    #[serde(rename = "TRACE_OPERATION_START")]
    TraceOperationStart,
    #[serde(rename = "TRACE_OPERATION_END")]
    TraceOperationEnd,
    #[serde(rename = "PUT_OPERATION_START")]
    PutOperationStart,
    #[serde(rename = "PUT_OPERATION_END")]
    PutOperationEnd,
    #[serde(rename = "POST_OPERATION_START")]
    PostOperationStart,
    #[serde(rename = "POST_OPERATION_END")]
    PostOperationEnd,
    #[serde(rename = "PATCH_OPERATION_START")]
    PatchOperationStart,
    #[serde(rename = "PATCH_OPERATION_END")]
    PatchOperationEnd,
    #[serde(rename = "OPTIONS_OPERATION_START")]
    OptionsOperationStart,
    #[serde(rename = "OPTIONS_OPERATION_END")]
    OptionsOperationEnd,
    #[serde(rename = "HEAD_OPERATION_START")]
    HeadOperationStart,
    #[serde(rename = "HEAD_OPERATION_END")]
    HeadOperationEnd,
    #[serde(rename = "GET_OPERATION_START")]
    GetOperationStart,
    #[serde(rename = "GET_OPERATION_END")]
    GetOperationEnd,
    #[serde(rename = "DELETE_OPERATION_START")]
    DeleteOperationStart,
    #[serde(rename = "DELETE_OPERATION_END")]
    DeleteOperationEnd,
    #[serde(rename = "COOKIE_PARAMETER_START")]
    CookieParameterStart,
    #[serde(rename = "COOKIE_PARAMETER_END")]
    CookieParameterEnd,
    #[serde(rename = "PARAMETERS_START")]
    ParametersStart,
    #[serde(rename = "PARAMETERS_END")]
    ParametersEnd,
    #[serde(rename = "RESPONSE_START")]
    ResponseStart,
    #[serde(rename = "RESPONSE_END")]
    ResponseEnd,
    #[serde(rename = "MEDIA_TYPES_START")]
    MediaTypesStart,
    #[serde(rename = "MEDIA_TYPES_END")]
    MediaTypesEnd,
    #[serde(rename = "LINKS_START")]
    LinksStart,
    #[serde(rename = "LINKS_END")]
    LinksEnd,
    #[serde(rename = "ASYNC_CALLBACKS_START")]
    AsyncCallbacksStart,
    #[serde(rename = "ASYNC_CALLBACKS_END")]
    AsyncCallbacksEnd,
    #[serde(rename = "ASYNC_CALLBACK_START")]
    AsyncCallbackStart,
    #[serde(rename = "ASYNC_CALLBACK_END")]
    AsyncCallbackEnd,
    #[serde(rename = "HEADERS_START")]
    HeadersStart,
    #[serde(rename = "HEADERS_END")]
    HeadersEnd,
    #[serde(rename = "SECURITY_SCHEMES_START")]
    SecuritySchemesStart,
    #[serde(rename = "SECURITY_SCHEMES_END")]
    SecuritySchemesEnd,
    #[serde(rename = "HEADER_START")]
    HeaderStart,
    #[serde(rename = "HEADER_END")]
    HeaderEnd,
    #[serde(rename = "REQUEST_BODY_START")]
    RequestBodyStart,
    #[serde(rename = "REQUEST_BODY_END")]
    RequestBodyEnd,
    #[serde(rename = "EXAMPLES_EXAMPLE")]
    ExamplesExample,
    #[serde(rename = "EXAMPLES_START")]
    ExamplesStart,
    #[serde(rename = "EXAMPLES_END")]
    ExamplesEnd,
    #[serde(rename = "REQUEST_BODIES_START")]
    RequestBodiesStart,
    #[serde(rename = "REQUEST_BODIES_END")]
    RequestBodiesEnd,
    #[serde(rename = "GENERIC_PARAMETERS_START")]
    GenericParametersStart,
    #[serde(rename = "GENERIC_PARAMETER")]
    GenericParameter,
    #[serde(rename = "GENERIC_PARAMETERS_END")]
    GenericParametersEnd,
    #[serde(rename = "PARAMETER_SCHEMA_OR_CONTENT_START")]
    ParameterSchemaOrContentStart,
    #[serde(rename = "PARAMETER_SCHEMA_OR_CONTENT_END")]
    ParameterSchemaOrContentEnd,
    #[serde(rename = "MEDIA_TYPE_START")]
    MediaTypeStart,
    #[serde(rename = "MEDIA_TYPE_END")]
    MediaTypeEnd,
    #[serde(rename = "LINK_START")]
    LinkStart,
    #[serde(rename = "LINK_END")]
    LinkEnd,
    #[serde(rename = "COMPONENTS_START")]
    ComponentsStart,
    #[serde(rename = "COMPONENTS_END")]
    ComponentsEnd,
    #[serde(rename = "MEDIA_TYPE_EXAMPLE")]
    GenericExample,
    #[serde(rename = "GENERIC_REQUEST_BODY")]
    GenericRequestBody,
    #[serde(rename = "ENCODING_START")]
    EncodingStart,
    #[serde(rename = "ENCODING_END")]
    EncodingEnd,
    #[serde(rename = "ENCODINGS_START")]
    EncodingsStart,
    #[serde(rename = "ENCODINGS_END")]
    EncodingsEnd,

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
    ServersStart,
    #[serde(rename = "SPEC_SERVERS_END")]
    ServersEnd,
    #[serde(rename = "SERVER_START")]
    ServerStart,
    #[serde(rename = "SERVER_END")]
    ServerEnd,
    #[serde(rename = "SERVER_VARIABLE")]
    ServerVariable,
    #[serde(rename = "SPEC_INFO_START")]
    SpecInfoStart,
    #[serde(rename = "SPEC_INFO_END")]
    SpecInfoEnd,
    #[serde(rename = "SPEC_INFO_CONTACT")]
    SpecInfoContact,
    #[serde(rename = "SPEC_INFO_LICENSE")]
    SpecInfoLicense,

    #[serde(rename = "SECURITY_REQUIREMENTS_START")]
    SecurityRequirementsStart,
    #[serde(rename = "SECURITY_REQUIREMENT")]
    SecurityRequirement,
    #[serde(rename = "SECURITY_REQUIREMENTS_END")]
    SecurityRequirementsEnd,
    #[serde(rename = "OBJECT_START")]
    ObjectStart,
    #[serde(rename = "OBJECT_PROPERTY_START")]
    ObjectPropertyStart,
    #[serde(rename = "OBJECT_PROPERTY_END")]
    ObjectPropertyEnd,
    #[serde(rename = "OBJECT_PROPERTIES_START")]
    ObjectPropertiesStart,
    #[serde(rename = "OBJECT_PROPERTIES_END")]
    ObjectPropertiesEnd,
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
            Script::ComponentsResponsesStart => "visitors/components/responses/responses_start",
            Script::ComponentsResponsesEnd => "visitors/components/responses/responses_end",
            Script::ResponseStart => "visitors/components/responses/response_start",
            Script::ResponseEnd => "visitors/components/responses/response_end",
            Script::HeadersStart => "visitors/common/headers/headers_start",
            Script::HeadersEnd => "visitors/common/headers/headers_end",
            Script::HeaderStart => "visitors/common/headers/header/header_start",
            Script::HeaderEnd => "visitors/common/headers/header/header_end",
            Script::ExamplesExample => "visitors/common/examples/examples_example",
            Script::ExamplesStart => "visitors/common/examples/examples_start",
            Script::ExamplesEnd => "visitors/common/examples/examples_end",
            Script::ParameterSchemaOrContentStart => {
                "visitors/common/parameter_schema_or_content/parameter_schema_or_content_start"
            }
            Script::ParameterSchemaOrContentEnd => {
                "visitors/common/parameter_schema_or_content/parameter_schema_or_content_end"
            }
            Script::MediaTypeStart => "visitors/common/media_types/media_type/media_type_start",
            Script::MediaTypeEnd => "visitors/common/media_types/media_type/media_type_end",
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

            Script::ServersStart => "visitors/common/servers/servers_start",
            Script::ServersEnd => "visitors/common/servers/servers_end",
            Script::ServerStart => "visitors/common/servers/server/server_start",
            Script::ServerEnd => "visitors/common/servers/server/server_end",
            Script::ServerVariable => "visitors/common/servers/server/server_variable",
            Script::SpecInfoStart => "visitors/info/spec_info_start",
            Script::SpecInfoEnd => "visitors/info/spec_info_end",

            Script::SpecInfoContact => "visitors/info/spec_info_contact",
            Script::SpecInfoLicense => "visitors/info/spec_info_license",

            Script::SecurityRequirementsStart => {
                "visitors/common/security_requirements/security_requirements_start"
            }
            Script::SecurityRequirement => {
                "visitors/common/security_requirements/security_requirement"
            }
            Script::SecurityRequirementsEnd => {
                "visitors/common/security_requirements/security_requirements_end"
            }

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
            Script::GenericExample => "visitors/common/generic_example/example",
            Script::EncodingsStart => "visitors/common/media_type/media_type_encodings_start",
            Script::EncodingsEnd => "visitors/common/media_type/media_type_encodings_end",
            Script::ObjectPropertiesStart => {
                "visitors/components/schemas/kind/type/object/object_properties_start"
            }
            Script::ObjectPropertiesEnd => {
                "visitors/components/schemas/kind/type/object/object_properties_end"
            }
            Script::MediaTypesStart => "visitors/common/media_types/media_types_start",
            Script::MediaTypesEnd => "visitors/common/media_types/media_types_end",
            Script::LinkStart => "visitors/common/links/link/link_start",
            Script::LinkEnd => "visitors/common/links/link/link_end",
            Script::GenericRequestBody => "visitors/common/generic_request_body/request_body",
            Script::GenericParametersStart => "visitors/common/generic_parameters/parameters_start",
            Script::GenericParametersEnd => "visitors/common/generic_parameters/parameters_end",
            Script::GenericParameter => "visitors/common/generic_parameters/parameter",
            Script::LinksStart => "visitors/common/links/links_start",
            Script::LinksEnd => "visitors/common/links/links_end",
            Script::ParametersStart => "visitors/common/parameters/parameters_start",
            Script::ParametersEnd => "visitors/common/parameters/parameters_end",
            Script::QueryParameterStart => {
                "visitors/common/parameters/query_parameter/query_parameter_start"
            }
            Script::QueryParameterEnd => {
                "visitors/common/parameters/query_parameter/query_parameter_end"
            }
            Script::ParameterDataStart => "visitors/common/parameter_data/parameter_data_start",
            Script::ParameterDataEnd => "visitors/common/parameter_data/parameter_data_end",
            Script::HeaderParameterStart => {
                "visitors/common/parameters/header_parameter/header_parameter_start"
            }
            Script::HeaderParameterEnd => {
                "visitors/common/parameters/header_parameter/header_parameter_end"
            }
            Script::PathParameterStart => {
                "visitors/common/parameters/path_parameter/path_parameter_start"
            }
            Script::PathParameterEnd => {
                "visitors/common/parameters/path_parameter/path_parameter_end"
            }
            Script::CookieParameterStart => {
                "visitors/common/parameters/cookie_parameter/cookie_parameter_start"
            }
            Script::CookieParameterEnd => {
                "visitors/common/parameters/cookie_parameter/cookie_parameter_end"
            }
            Script::RequestBodiesStart => "visitors/components/request_bodies/request_bodies_start",
            Script::RequestBodiesEnd => "visitors/components/request_bodies/request_bodies_end",
            Script::RequestBodyStart => "visitors/components/request_bodies/request_body_start",
            Script::RequestBodyEnd => "visitors/components/request_bodies/request_body_end",
            Script::SecuritySchemesStart => {
                "visitors/components/security_schemes/security_schemes_start"
            }
            Script::SecuritySchemesEnd => {
                "visitors/components/security_schemes/security_schemes_end"
            }
            Script::SecuritySchemeApiKey => "visitors/components/security_schemes/api_key",
            Script::SecuritySchemeHttp => "visitors/components/security_schemes/http",
            Script::SecuritySchemeOAuth2Start => {
                "visitors/components/security_schemes/oauth2_start"
            }
            Script::SecuritySchemeOAuth2End => "visitors/components/security_schemes/oauth2_end",
            Script::SecuritySchemeOAuth2FlowsStart => {
                "visitors/components/security_schemes/oauth2_flows_start"
            }
            Script::SecuritySchemeOAuth2FlowsEnd => {
                "visitors/components/security_schemes/oauth2_flows_end"
            }
            Script::SecuritySchemeOAuth2FlowImplicit => {
                "visitors/components/security_schemes/oauth2_flow_implicit"
            }
            Script::SecuritySchemeOAuth2FlowPassword => {
                "visitors/components/security_schemes/oauth2_flow_password"
            }
            Script::SecuritySchemeOAuth2FlowClientCredentials => {
                "visitors/components/security_schemes/oauth2_flow_client_credentials"
            }
            Script::SecuritySchemeOAuth2FlowAuthorizationCode => {
                "visitors/components/security_schemes/oauth2_flow_authorization_code"
            }
            Script::SecuritySchemeOpenIdConnect => {
                "visitors/components/security_schemes/openid_connect"
            }
            Script::ComponentsStart => "visitors/components/components_start",
            Script::ComponentsEnd => "visitors/components/components_end",
            Script::AsyncCallbacksStart => {
                "visitors/components/async_callbacks/async_callbacks_start"
            }
            Script::AsyncCallbacksEnd => "visitors/components/async_callbacks/async_callbacks_end",
            Script::AsyncCallbackStart => "visitors/common/async_callback/async_callback_start",
            Script::AsyncCallbackEnd => "visitors/common/async_callback/async_callback_end",
            Script::PathItemStart => "visitors/common/path_item/path_item_start",
            Script::PathItemEnd => "visitors/common/path_item/path_item_end",
            Script::OperationResponsesStart => {
                "visitors/common/operation/responses/responses_start"
            }
            Script::OperationResponsesEnd => "visitors/common/operation/responses/responses_end",
            Script::TraceOperationStart => "visitors/common/operation/trace_operation_start",
            Script::TraceOperationEnd => "visitors/common/operation/trace_operation_end",
            Script::PutOperationStart => "visitors/common/operation/put_operation_start",
            Script::PutOperationEnd => "visitors/common/operation/put_operation_end",
            Script::PostOperationStart => "visitors/common/operation/post_operation_start",
            Script::PostOperationEnd => "visitors/common/operation/post_operation_end",
            Script::PatchOperationStart => "visitors/common/operation/patch_operation_start",
            Script::PatchOperationEnd => "visitors/common/operation/patch_operation_end",
            Script::OptionsOperationStart => "visitors/common/operation/options_operation_start",
            Script::OptionsOperationEnd => "visitors/common/operation/options_operation_end",
            Script::HeadOperationStart => "visitors/common/operation/head_operation_start",
            Script::HeadOperationEnd => "visitors/common/operation/head_operation_end",
            Script::GetOperationStart => "visitors/common/operation/get_operation_start",
            Script::GetOperationEnd => "visitors/common/operation/get_operation_end",
            Script::DeleteOperationStart => "visitors/common/operation/delete_operation_start",
            Script::DeleteOperationEnd => "visitors/common/operation/delete_operation_end",
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
