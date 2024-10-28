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
#[serde(rename_all = "camelCase")]
pub enum Script {
    Prelude,
    VisitOperationResponsesStart,
    VisitOperationResponsesEnd,
    VisitResponsesStart,
    VisitResponsesEnd,
    VisitParameterDataStart,
    VisitParameterDataEnd,
    VisitSecuritySchemeApiKey,
    VisitSecuritySchemeOpenIdConnect,
    VisitSecuritySchemeOAuth2FlowImplicit,
    VisitSecuritySchemeOAuth2FlowPassword,
    VisitSecuritySchemeOAuth2FlowClientCredentials,
    VisitSecuritySchemeOAuth2FlowAuthorizationCode,
    VisitSecuritySchemeHttp,
    VisitSecuritySchemeOAuth2Start,
    VisitSecuritySchemeOAuth2End,
    VisitSecuritySchemeOAuth2FlowsStart,
    VisitSecuritySchemeOAuth2FlowsEnd,
    VisitQueryParameterStart,
    VisitQueryParameterEnd,
    VisitHeaderParameterStart,
    VisitHeaderParameterEnd,
    VisitPathParameterStart,
    VisitPathParameterEnd,
    VisitPathItemStart,
    VisitPathItemEnd,
    VisitTraceOperationStart,
    VisitTraceOperationEnd,
    VisitPutOperationStart,
    VisitPutOperationEnd,
    VisitPostOperationStart,
    VisitPostOperationEnd,
    VisitPatchOperationStart,
    VisitPatchOperationEnd,
    VisitOptionsOperationStart,
    VisitOptionsOperationEnd,
    VisitHeadOperationStart,
    VisitHeadOperationEnd,
    VisitGetOperationStart,
    VisitGetOperationEnd,
    VisitDeleteOperationStart,
    VisitDeleteOperationEnd,
    VisitCookieParameterStart,
    VisitCookieParameterEnd,
    VisitParametersStart,
    VisitParametersEnd,
    VisitPathsStart,
    VisitPathsEnd,
    VisitResponseStart,
    VisitResponseEnd,
    VisitMediaTypesStart,
    VisitMediaTypesEnd,
    VisitLinksStart,
    VisitLinksEnd,
    VisitAsyncCallbacksStart,
    VisitAsyncCallbacksEnd,
    VisitAsyncCallbackStart,
    VisitAsyncCallbackEnd,
    VisitHeadersStart,
    VisitHeadersEnd,
    VisitSecuritySchemesStart,
    VisitSecuritySchemesEnd,
    VisitHeaderStart,
    VisitHeaderEnd,
    VisitRequestBodyStart,
    VisitRequestBodyEnd,
    VisitExampleStart,
    VisitExampleEnd,
    VisitExamplesStart,
    VisitExamplesEnd,
    VisitRequestBodiesStart,
    VisitRequestBodiesEnd,
    VisitGenericParametersStart,
    VisitGenericParameter,
    VisitGenericParametersEnd,
    VisitParameterSchemaOrContentStart,
    VisitParameterSchemaOrContentEnd,
    VisitMediaTypeStart,
    VisitMediaTypeEnd,
    VisitLinkStart,
    VisitLinkEnd,
    VisitComponentsStart,
    VisitComponentsEnd,
    VisitGenericExample,
    VisitGenericRequestBody,
    VisitEncodingStart,
    VisitEncodingEnd,
    VisitEncodingsStart,
    VisitEncodingsEnd,
    VisitSchemasStart,
    VisitSchemasEnd,
    VisitSchemaStart,
    VisitSchemaEnd,
    VisitDefault,
    VisitDiscriminator,
    VisitSpecStart,
    VisitSpecEnd,
    VisitExternalDocs,
    VisitSpecTag,
    VisitSpecTagsEnd,
    VisitSpecTagsStart,
    VisitServersStart,
    VisitServersEnd,
    VisitServerStart,
    VisitServerEnd,
    VisitServerVariable,
    VisitSpecInfoStart,
    VisitSpecInfoEnd,
    VisitSpecInfoContact,
    VisitSpecInfoLicense,
    VisitSecurityRequirementsStart,
    VisitSecurityRequirement,
    VisitSecurityRequirementsEnd,
    VisitObjectStart,
    VisitObjectPropertyStart,
    VisitObjectPropertyEnd,
    VisitObjectPropertiesStart,
    VisitObjectPropertiesEnd,
    VisitObjectEnd,
    VisitAnySchema,
    VisitPropertyNotStart,
    VisitPropertyNotEnd,
    VisitAdditionalPropertiesAny,
    VisitAdditionalPropertiesStart,
    VisitAdditionalPropertiesEnd,
    VisitStringProperty,
    VisitNumberProperty,
    VisitIntegerProperty,
    VisitArrayPropertyStart,
    VisitArrayPropertyEnd,
    VisitBooleanProperty,
    VisitOneOfStart,
    VisitOneOfEnd,
    VisitAllOfStart,
    VisitAllOfEnd,
    VisitAnyOfStart,
    VisitAnyOfEnd,
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
            Script::VisitResponsesStart => "visitors/components/responses/visitResponsesStart",
            Script::VisitResponsesEnd => "visitors/components/responses/visitResponsesEnd",
            Script::VisitResponseStart => "visitors/components/responses/visitResponseStart",
            Script::VisitResponseEnd => "visitors/components/responses/visitResponseEnd",
            Script::VisitHeadersStart => "visitors/common/headers/visitHeadersStart",
            Script::VisitHeadersEnd => "visitors/common/headers/visitHeadersEnd",
            Script::VisitHeaderStart => "visitors/common/headers/header/visitHeaderStart",
            Script::VisitHeaderEnd => "visitors/common/headers/header/visitHeaderEnd",
            Script::VisitExamplesStart => "visitors/common/examples/visitExamplesStart",
            Script::VisitExampleStart => "visitors/common/examples/visitExampleStart",
            Script::VisitExampleEnd => "visitors/common/examples/visitExampleEnd",
            Script::VisitExamplesEnd => "visitors/common/examples/visitExamplesEnd",
            Script::VisitParameterSchemaOrContentStart => "visitors/common/parameter_schema_or_content/visitParameterSchemaOrContentStart",
            Script::VisitParameterSchemaOrContentEnd => "visitors/common/parameter_schema_or_content/visitParameterSchemaOrContentEnd",
            Script::VisitMediaTypeStart => "visitors/common/media_types/media_type/visitMediaTypeStart",
            Script::VisitMediaTypeEnd => "visitors/common/media_types/media_type/visitMediaTypeEnd",
            Script::VisitEncodingStart => "visitors/common/media_types/media_type/encoding/visitEncodingStart",
            Script::VisitEncodingEnd => "visitors/common/media_types/media_type/encoding/visitEncodingEnd",
            Script::VisitSchemasStart => "visitors/components/schemas/visitSchemasStart",
            Script::VisitSchemasEnd => "visitors/components/schemas/visitSchemasEnd",
            Script::VisitSchemaStart => "visitors/components/schemas/visitSchemaStart",
            Script::VisitSchemaEnd => "visitors/components/schemas/visitSchemaEnd",
            Script::VisitDefault => "visitors/components/schemas/visitDefault",
            Script::VisitDiscriminator => "visitors/components/schemas/visitDiscriminator",
            Script::VisitSpecStart => "visitors/visitSpecStart",
            Script::VisitSpecEnd => "visitors/visitSpecEnd",
            Script::VisitExternalDocs => "visitors/common/external_docs/visitExternalDocs",
            Script::VisitSpecTag => "visitors/tags/visitSpecTag",
            Script::VisitSpecTagsStart => "visitors/tags/visitSpecTagsStart",
            Script::VisitSpecTagsEnd => "visitors/tags/visitSpecTagsEnd",
            Script::VisitServersStart => "visitors/common/servers/visitServersStart",
            Script::VisitServersEnd => "visitors/common/servers/visitServersEnd",
            Script::VisitServerStart => "visitors/common/servers/server/visitServerStart",
            Script::VisitServerEnd => "visitors/common/servers/server/visitServerEnd",
            Script::VisitServerVariable => "visitors/common/servers/server/visitServerVariable",
            Script::VisitSpecInfoStart => "visitors/info/visitSpecInfoStart",
            Script::VisitSpecInfoEnd => "visitors/info/visitSpecInfoEnd",
            Script::VisitSpecInfoContact => "visitors/info/visitSpecInfoContact",
            Script::VisitSpecInfoLicense => "visitors/info/visitSpecInfoLicense",
            Script::VisitSecurityRequirementsStart => "visitors/common/security_requirements/visitSecurityRequirementsStart",
            Script::VisitSecurityRequirementsEnd => "visitors/common/security_requirements/visitSecurityRequirementsEnd",
            Script::VisitSecurityRequirement => "visitors/common/security_requirements/visitSecurityRequirement",
            Script::VisitObjectStart => "visitors/components/schemas/kind/type/object/visitObjectStart",
            Script::VisitObjectEnd => "visitors/components/schemas/kind/type/object/visitObjectEnd",
            Script::VisitAnySchema => "visitors/components/schemas/kind/any/visitAnySchema",
            Script::VisitPropertyNotStart => "visitors/components/schemas/kind/not/visitPropertyNotStart",
            Script::VisitPropertyNotEnd => "visitors/components/schemas/kind/not/visitPropertyNotEnd",
            Script::VisitAdditionalPropertiesAny => "visitors/components/schemas/kind/type/object/visitAdditionalPropertiesAny",
            Script::VisitStringProperty => "visitors/components/schemas/kind/type/string/visitStringProperty",
            Script::VisitNumberProperty => "visitors/components/schemas/kind/type/number/visitNumberProperty",
            Script::VisitIntegerProperty => "visitors/components/schemas/kind/type/integer/visitIntegerProperty",
            Script::VisitArrayPropertyStart => "visitors/components/schemas/kind/type/object/visitArrayPropertyStart",
            Script::VisitArrayPropertyEnd => "visitors/components/schemas/kind/type/object/visitArrayPropertyEnd",
            Script::VisitBooleanProperty => "visitors/components/schemas/kind/type/boolean/visitBooleanProperty",
            Script::VisitOneOfStart => "visitors/components/schemas/kind/oneOf/visitOneOfStart",
            Script::VisitOneOfEnd => "visitors/components/schemas/kind/oneOf/visitOneOfEnd",
            Script::VisitAllOfStart => "visitors/components/schemas/kind/allOf/visitAllOfStart",
            Script::VisitAllOfEnd => "visitors/components/schemas/kind/allOf/visitAllOfEnd",
            Script::VisitAnyOfStart => "visitors/components/schemas/kind/anyOf/visitAnyOfStart",
            Script::VisitAnyOfEnd => "visitors/components/schemas/kind/anyOf/visitAnyOfEnd",
            Script::VisitAdditionalPropertiesStart => "visitors/components/schemas/kind/type/object/visitAdditionalPropertiesStart",
            Script::VisitAdditionalPropertiesEnd => "visitors/components/schemas/kind/type/object/visitAdditionalPropertiesEnd",
            Script::VisitObjectPropertyStart => "visitors/components/schemas/kind/type/object/visitObjectPropertyStart",
            Script::VisitObjectPropertyEnd => "visitors/components/schemas/kind/type/object/visitObjectPropertyEnd",
            Script::VisitGenericExample => "visitors/common/generic_example/visitGenericExample",
            Script::VisitEncodingsStart => "visitors/common/media_types/media_type/visitMediaTypeEncodingsStart",
            Script::VisitEncodingsEnd => "visitors/common/media_types/media_type/visitMediaTypeEncodingsEnd",
            Script::VisitObjectPropertiesStart => "visitors/components/schemas/kind/type/object/visitObjectPropertiesStart",
            Script::VisitObjectPropertiesEnd => "visitors/components/schemas/kind/type/object/visitObjectPropertiesEnd",
            Script::VisitMediaTypesStart => "visitors/common/media_types/visitMediaTypesStart",
            Script::VisitMediaTypesEnd => "visitors/common/media_types/visitMediaTypesEnd",
            Script::VisitLinkStart => "visitors/common/links/link/visitLinkStart",
            Script::VisitLinkEnd => "visitors/common/links/link/visitLinkEnd",
            Script::VisitGenericRequestBody => "visitors/common/generic_request_body/visitGenericRequestBody",
            Script::VisitGenericParametersStart => "visitors/common/generic_parameters/visitGenericParametersStart",
            Script::VisitGenericParametersEnd => "visitors/common/generic_parameters/visitGenericParametersEnd",
            Script::VisitGenericParameter => "visitors/common/generic_parameters/visitGenericParameter",
            Script::VisitLinksStart => "visitors/common/links/visitLinksStart",
            Script::VisitLinksEnd => "visitors/common/links/visitLinksEnd",
            Script::VisitParametersStart => "visitors/common/parameters/visitParametersStart",
            Script::VisitParametersEnd => "visitors/common/parameters/visitParametersEnd",
            Script::VisitQueryParameterStart => "visitors/common/parameters/query_parameter/visitQueryParameterStart",
            Script::VisitQueryParameterEnd => "visitors/common/parameters/query_parameter/visitQueryParameterEnd",
            Script::VisitParameterDataStart => "visitors/common/parameter_data/visitParameterDataStart",
            Script::VisitParameterDataEnd => "visitors/common/parameter_data/visitParameterDataEnd",
            Script::VisitHeaderParameterStart => "visitors/common/parameters/header_parameter/visitHeaderParameterStart",
            Script::VisitHeaderParameterEnd => "visitors/common/parameters/header_parameter/visitHeaderParameterEnd",
            Script::VisitPathParameterStart => "visitors/common/parameters/path_parameter/visitPathParameterStart",
            Script::VisitPathParameterEnd => "visitors/common/parameters/path_parameter/visitPathParameterEnd",
            Script::VisitCookieParameterStart => "visitors/common/parameters/cookie_parameter/visitCookieParameterStart",
            Script::VisitCookieParameterEnd => "visitors/common/parameters/cookie_parameter/visitCookieParameterEnd",
            Script::VisitRequestBodiesStart => "visitors/components/request_bodies/visitRequestBodiesStart",
            Script::VisitRequestBodiesEnd => "visitors/components/request_bodies/visitRequestBodiesEnd",
            Script::VisitRequestBodyStart => "visitors/components/request_bodies/visitRequestBodyStart",
            Script::VisitRequestBodyEnd => "visitors/components/request_bodies/visitRequestBodyEnd",
            Script::VisitSecuritySchemesStart => "visitors/components/security_schemes/visitSecuritySchemesStart",
            Script::VisitSecuritySchemesEnd => "visitors/components/security_schemes/visitSecuritySchemesEnd",
            Script::VisitSecuritySchemeApiKey => "visitors/components/security_schemes/visitSecuritySchemeApiKey",
            Script::VisitSecuritySchemeHttp => "visitors/components/security_schemes/visitSecuritySchemeHttp",
            Script::VisitSecuritySchemeOAuth2Start => "visitors/components/security_schemes/visitSecuritySchemeOAuth2Start",
            Script::VisitSecuritySchemeOAuth2End => "visitors/components/security_schemes/visitSecuritySchemeOAuth2End",
            Script::VisitSecuritySchemeOAuth2FlowsStart => "visitors/components/security_schemes/visitSecuritySchemeOAuth2FlowsStart",
            Script::VisitSecuritySchemeOAuth2FlowsEnd => "visitors/components/security_schemes/visitSecuritySchemeOAuth2FlowsEnd",
            Script::VisitSecuritySchemeOAuth2FlowImplicit => "visitors/components/security_schemes/visitSecuritySchemeOAuth2FlowImplicit",
            Script::VisitSecuritySchemeOAuth2FlowPassword => "visitors/components/security_schemes/visitSecuritySchemeOAuth2FlowPassword",
            Script::VisitSecuritySchemeOAuth2FlowClientCredentials => "visitors/components/security_schemes/visitSecuritySchemeOAuth2FlowClientCredentials",
            Script::VisitSecuritySchemeOAuth2FlowAuthorizationCode => "visitors/components/security_schemes/visitSecuritySchemeOAuth2FlowAuthorizationCode",
            Script::VisitSecuritySchemeOpenIdConnect => "visitors/components/security_schemes/visitSecuritySchemeOpenIdConnect",
            Script::VisitComponentsStart => "visitors/components/visitComponentsStart",
            Script::VisitComponentsEnd => "visitors/components/visitComponentsEnd",
            Script::VisitAsyncCallbacksStart => "visitors/components/async_callbacks/visitAsyncCallbacksStart",
            Script::VisitAsyncCallbacksEnd => "visitors/components/async_callbacks/visitAsyncCallbacksEnd",
            Script::VisitAsyncCallbackStart => "visitors/common/async_callback/visitAsyncCallbackStart",
            Script::VisitAsyncCallbackEnd => "visitors/common/async_callback/visitAsyncCallbackEnd",
            Script::VisitPathItemStart => "visitors/common/path_item/visitPathItemStart",
            Script::VisitPathItemEnd => "visitors/common/path_item/visitPathItemEnd",
            Script::VisitOperationResponsesStart => "visitors/common/operation/responses/visitOperationResponsesStart",
            Script::VisitOperationResponsesEnd => "visitors/common/operation/responses/visitOperationResponsesEnd",
            Script::VisitTraceOperationStart => "visitors/common/operation/visitTraceOperationStart",
            Script::VisitTraceOperationEnd => "visitors/common/operation/visitTraceOperationEnd",
            Script::VisitPutOperationStart => "visitors/common/operation/visitPutOperationStart",
            Script::VisitPutOperationEnd => "visitors/common/operation/visitPutOperationEnd",
            Script::VisitPostOperationStart => "visitors/common/operation/visitPostOperationStart",
            Script::VisitPostOperationEnd => "visitors/common/operation/visitPostOperationEnd",
            Script::VisitPatchOperationStart => "visitors/common/operation/visitPatchOperationStart",
            Script::VisitPatchOperationEnd => "visitors/common/operation/visitPatchOperationEnd",
            Script::VisitOptionsOperationStart => "visitors/common/operation/visitOptionsOperationStart",
            Script::VisitOptionsOperationEnd => "visitors/common/operation/visitOptionsOperationEnd",
            Script::VisitHeadOperationStart => "visitors/common/operation/visitHeadOperationStart",
            Script::VisitHeadOperationEnd => "visitors/common/operation/visitHeadOperationEnd",
            Script::VisitGetOperationStart => "visitors/common/operation/visitGetOperationStart",
            Script::VisitGetOperationEnd => "visitors/common/operation/visitGetOperationEnd",
            Script::VisitDeleteOperationStart => "visitors/common/operation/visitDeleteOperationStart",
            Script::VisitDeleteOperationEnd => "visitors/common/operation/visitDeleteOperationEnd",
            Script::VisitPathsStart => "visitors/paths/visitPathsStart",
            Script::VisitPathsEnd => "visitors/paths/visitPathsEnd",
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
