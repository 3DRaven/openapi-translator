use std::{fmt::Display, ops::Add, path::Path};

use anyhow::{anyhow, Context, Result};
use mlua::LuaSerdeExt;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

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

#[derive(Clone, Serialize, Deserialize, EnumIter, PartialEq, Eq)]
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
            Script::VisitResponsesStart => "components/responses/visitResponsesStart",
            Script::VisitResponsesEnd => "components/responses/visitResponsesEnd",
            Script::VisitResponseStart => "components/responses/visitResponseStart",
            Script::VisitResponseEnd => "components/responses/visitResponseEnd",
            Script::VisitHeadersStart => "common/headers/visitHeadersStart",
            Script::VisitHeadersEnd => "common/headers/visitHeadersEnd",
            Script::VisitHeaderStart => "common/headers/header/visitHeaderStart",
            Script::VisitHeaderEnd => "common/headers/header/visitHeaderEnd",
            Script::VisitExamplesStart => "common/examples/visitExamplesStart",
            Script::VisitExampleStart => "common/examples/visitExampleStart",
            Script::VisitExampleEnd => "common/examples/visitExampleEnd",
            Script::VisitExamplesEnd => "common/examples/visitExamplesEnd",
            Script::VisitParameterSchemaOrContentStart => {
                "common/parameter_schema_or_content/visitParameterSchemaOrContentStart"
            }
            Script::VisitParameterSchemaOrContentEnd => {
                "common/parameter_schema_or_content/visitParameterSchemaOrContentEnd"
            }
            Script::VisitMediaTypeStart => "common/media_types/media_type/visitMediaTypeStart",
            Script::VisitMediaTypeEnd => "common/media_types/media_type/visitMediaTypeEnd",
            Script::VisitEncodingStart => {
                "common/media_types/media_type/encoding/visitEncodingStart"
            }
            Script::VisitEncodingEnd => "common/media_types/media_type/encoding/visitEncodingEnd",
            Script::VisitSchemasStart => "components/schemas/visitSchemasStart",
            Script::VisitSchemasEnd => "components/schemas/visitSchemasEnd",
            Script::VisitSchemaStart => "components/schemas/visitSchemaStart",
            Script::VisitSchemaEnd => "components/schemas/visitSchemaEnd",
            Script::VisitDefault => "components/schemas/visitDefault",
            Script::VisitDiscriminator => "components/schemas/visitDiscriminator",
            Script::VisitSpecStart => "visitSpecStart",
            Script::VisitSpecEnd => "visitSpecEnd",
            Script::VisitExternalDocs => "common/external_docs/visitExternalDocs",
            Script::VisitSpecTag => "tags/visitSpecTag",
            Script::VisitSpecTagsStart => "tags/visitSpecTagsStart",
            Script::VisitSpecTagsEnd => "tags/visitSpecTagsEnd",
            Script::VisitServersStart => "common/servers/visitServersStart",
            Script::VisitServersEnd => "common/servers/visitServersEnd",
            Script::VisitServerStart => "common/servers/server/visitServerStart",
            Script::VisitServerEnd => "common/servers/server/visitServerEnd",
            Script::VisitServerVariable => "common/servers/server/visitServerVariable",
            Script::VisitSpecInfoStart => "info/visitSpecInfoStart",
            Script::VisitSpecInfoEnd => "info/visitSpecInfoEnd",
            Script::VisitSpecInfoContact => "info/visitSpecInfoContact",
            Script::VisitSpecInfoLicense => "info/visitSpecInfoLicense",
            Script::VisitSecurityRequirementsStart => {
                "common/security_requirements/visitSecurityRequirementsStart"
            }
            Script::VisitSecurityRequirementsEnd => {
                "common/security_requirements/visitSecurityRequirementsEnd"
            }
            Script::VisitSecurityRequirement => {
                "common/security_requirements/visitSecurityRequirement"
            }
            Script::VisitObjectStart => "components/schemas/kind/type/object/visitObjectStart",
            Script::VisitObjectEnd => "components/schemas/kind/type/object/visitObjectEnd",
            Script::VisitAnySchema => "components/schemas/kind/any/visitAnySchema",
            Script::VisitPropertyNotStart => "components/schemas/kind/not/visitPropertyNotStart",
            Script::VisitPropertyNotEnd => "components/schemas/kind/not/visitPropertyNotEnd",
            Script::VisitAdditionalPropertiesAny => {
                "components/schemas/kind/type/object/visitAdditionalPropertiesAny"
            }
            Script::VisitStringProperty => {
                "components/schemas/kind/type/string/visitStringProperty"
            }
            Script::VisitNumberProperty => {
                "components/schemas/kind/type/number/visitNumberProperty"
            }
            Script::VisitIntegerProperty => {
                "components/schemas/kind/type/integer/visitIntegerProperty"
            }
            Script::VisitArrayPropertyStart => {
                "components/schemas/kind/type/object/visitArrayPropertyStart"
            }
            Script::VisitArrayPropertyEnd => {
                "components/schemas/kind/type/object/visitArrayPropertyEnd"
            }
            Script::VisitBooleanProperty => {
                "components/schemas/kind/type/boolean/visitBooleanProperty"
            }
            Script::VisitOneOfStart => "components/schemas/kind/oneOf/visitOneOfStart",
            Script::VisitOneOfEnd => "components/schemas/kind/oneOf/visitOneOfEnd",
            Script::VisitAllOfStart => "components/schemas/kind/allOf/visitAllOfStart",
            Script::VisitAllOfEnd => "components/schemas/kind/allOf/visitAllOfEnd",
            Script::VisitAnyOfStart => "components/schemas/kind/anyOf/visitAnyOfStart",
            Script::VisitAnyOfEnd => "components/schemas/kind/anyOf/visitAnyOfEnd",
            Script::VisitAdditionalPropertiesStart => {
                "components/schemas/kind/type/object/visitAdditionalPropertiesStart"
            }
            Script::VisitAdditionalPropertiesEnd => {
                "components/schemas/kind/type/object/visitAdditionalPropertiesEnd"
            }
            Script::VisitObjectPropertyStart => {
                "components/schemas/kind/type/object/visitObjectPropertyStart"
            }
            Script::VisitObjectPropertyEnd => {
                "components/schemas/kind/type/object/visitObjectPropertyEnd"
            }
            Script::VisitGenericExample => "common/generic_example/visitGenericExample",
            Script::VisitEncodingsStart => {
                "common/media_types/media_type/visitMediaTypeEncodingsStart"
            }
            Script::VisitEncodingsEnd => "common/media_types/media_type/visitMediaTypeEncodingsEnd",
            Script::VisitObjectPropertiesStart => {
                "components/schemas/kind/type/object/visitObjectPropertiesStart"
            }
            Script::VisitObjectPropertiesEnd => {
                "components/schemas/kind/type/object/visitObjectPropertiesEnd"
            }
            Script::VisitMediaTypesStart => "common/media_types/visitMediaTypesStart",
            Script::VisitMediaTypesEnd => "common/media_types/visitMediaTypesEnd",
            Script::VisitLinkStart => "common/links/link/visitLinkStart",
            Script::VisitLinkEnd => "common/links/link/visitLinkEnd",
            Script::VisitGenericRequestBody => {
                "common/generic_request_body/visitGenericRequestBody"
            }
            Script::VisitGenericParametersStart => {
                "common/generic_parameters/visitGenericParametersStart"
            }
            Script::VisitGenericParametersEnd => {
                "common/generic_parameters/visitGenericParametersEnd"
            }
            Script::VisitGenericParameter => "common/generic_parameters/visitGenericParameter",
            Script::VisitLinksStart => "common/links/visitLinksStart",
            Script::VisitLinksEnd => "common/links/visitLinksEnd",
            Script::VisitParametersStart => "common/parameters/visitParametersStart",
            Script::VisitParametersEnd => "common/parameters/visitParametersEnd",
            Script::VisitQueryParameterStart => {
                "common/parameters/query_parameter/visitQueryParameterStart"
            }
            Script::VisitQueryParameterEnd => {
                "common/parameters/query_parameter/visitQueryParameterEnd"
            }
            Script::VisitParameterDataStart => "common/parameter_data/visitParameterDataStart",
            Script::VisitParameterDataEnd => "common/parameter_data/visitParameterDataEnd",
            Script::VisitHeaderParameterStart => {
                "common/parameters/header_parameter/visitHeaderParameterStart"
            }
            Script::VisitHeaderParameterEnd => {
                "common/parameters/header_parameter/visitHeaderParameterEnd"
            }
            Script::VisitPathParameterStart => {
                "common/parameters/path_parameter/visitPathParameterStart"
            }
            Script::VisitPathParameterEnd => {
                "common/parameters/path_parameter/visitPathParameterEnd"
            }
            Script::VisitCookieParameterStart => {
                "common/parameters/cookie_parameter/visitCookieParameterStart"
            }
            Script::VisitCookieParameterEnd => {
                "common/parameters/cookie_parameter/visitCookieParameterEnd"
            }
            Script::VisitRequestBodiesStart => "components/request_bodies/visitRequestBodiesStart",
            Script::VisitRequestBodiesEnd => "components/request_bodies/visitRequestBodiesEnd",
            Script::VisitRequestBodyStart => "components/request_bodies/visitRequestBodyStart",
            Script::VisitRequestBodyEnd => "components/request_bodies/visitRequestBodyEnd",
            Script::VisitSecuritySchemesStart => {
                "components/security_schemes/visitSecuritySchemesStart"
            }
            Script::VisitSecuritySchemesEnd => {
                "components/security_schemes/visitSecuritySchemesEnd"
            }
            Script::VisitSecuritySchemeApiKey => {
                "components/security_schemes/visitSecuritySchemeApiKey"
            }
            Script::VisitSecuritySchemeHttp => {
                "components/security_schemes/visitSecuritySchemeHttp"
            }
            Script::VisitSecuritySchemeOAuth2Start => {
                "components/security_schemes/visitSecuritySchemeOAuth2Start"
            }
            Script::VisitSecuritySchemeOAuth2End => {
                "components/security_schemes/visitSecuritySchemeOAuth2End"
            }
            Script::VisitSecuritySchemeOAuth2FlowsStart => {
                "components/security_schemes/visitSecuritySchemeOAuth2FlowsStart"
            }
            Script::VisitSecuritySchemeOAuth2FlowsEnd => {
                "components/security_schemes/visitSecuritySchemeOAuth2FlowsEnd"
            }
            Script::VisitSecuritySchemeOAuth2FlowImplicit => {
                "components/security_schemes/visitSecuritySchemeOAuth2FlowImplicit"
            }
            Script::VisitSecuritySchemeOAuth2FlowPassword => {
                "components/security_schemes/visitSecuritySchemeOAuth2FlowPassword"
            }
            Script::VisitSecuritySchemeOAuth2FlowClientCredentials => {
                "components/security_schemes/visitSecuritySchemeOAuth2FlowClientCredentials"
            }
            Script::VisitSecuritySchemeOAuth2FlowAuthorizationCode => {
                "components/security_schemes/visitSecuritySchemeOAuth2FlowAuthorizationCode"
            }
            Script::VisitSecuritySchemeOpenIdConnect => {
                "components/security_schemes/visitSecuritySchemeOpenIdConnect"
            }
            Script::VisitComponentsStart => "components/visitComponentsStart",
            Script::VisitComponentsEnd => "components/visitComponentsEnd",
            Script::VisitAsyncCallbacksStart => {
                "components/async_callbacks/visitAsyncCallbacksStart"
            }
            Script::VisitAsyncCallbacksEnd => "components/async_callbacks/visitAsyncCallbacksEnd",
            Script::VisitAsyncCallbackStart => "common/async_callback/visitAsyncCallbackStart",
            Script::VisitAsyncCallbackEnd => "common/async_callback/visitAsyncCallbackEnd",
            Script::VisitPathItemStart => "common/path_item/visitPathItemStart",
            Script::VisitPathItemEnd => "common/path_item/visitPathItemEnd",
            Script::VisitOperationResponsesStart => {
                "common/operation/responses/visitOperationResponsesStart"
            }
            Script::VisitOperationResponsesEnd => {
                "common/operation/responses/visitOperationResponsesEnd"
            }
            Script::VisitTraceOperationStart => "common/operation/visitTraceOperationStart",
            Script::VisitTraceOperationEnd => "common/operation/visitTraceOperationEnd",
            Script::VisitPutOperationStart => "common/operation/visitPutOperationStart",
            Script::VisitPutOperationEnd => "common/operation/visitPutOperationEnd",
            Script::VisitPostOperationStart => "common/operation/visitPostOperationStart",
            Script::VisitPostOperationEnd => "common/operation/visitPostOperationEnd",
            Script::VisitPatchOperationStart => "common/operation/visitPatchOperationStart",
            Script::VisitPatchOperationEnd => "common/operation/visitPatchOperationEnd",
            Script::VisitOptionsOperationStart => "common/operation/visitOptionsOperationStart",
            Script::VisitOptionsOperationEnd => "common/operation/visitOptionsOperationEnd",
            Script::VisitHeadOperationStart => "common/operation/visitHeadOperationStart",
            Script::VisitHeadOperationEnd => "common/operation/visitHeadOperationEnd",
            Script::VisitGetOperationStart => "common/operation/visitGetOperationStart",
            Script::VisitGetOperationEnd => "common/operation/visitGetOperationEnd",
            Script::VisitDeleteOperationStart => "common/operation/visitDeleteOperationStart",
            Script::VisitDeleteOperationEnd => "common/operation/visitDeleteOperationEnd",
            Script::VisitPathsStart => "paths/visitPathsStart",
            Script::VisitPathsEnd => "paths/visitPathsEnd",
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
