use std::{fmt::Display, ops::Add, path::Path};

use anyhow::{anyhow, Context, Result};
use mlua::LuaSerdeExt;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::{
    holders::context::get_lua_vm,
    services::{code, scripts},
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
    Target,
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
    VisitParameterReferenceStart,
    VisitParameterReferenceEnd,
    VisitQueryParameterStart,
    VisitQueryParameterEnd,
    VisitHeaderParameterStart,
    VisitHeaderParameterEnd,
    VisitPathParameterStart,
    VisitPathParameterEnd,
    VisitPathItemReferenceStart,
    VisitPathItemReferenceEnd,
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
    VisitResponseReferenceStart,
    VisitResponseReferenceEnd,
    VisitResponseStart,
    VisitResponseEnd,
    VisitMediaTypesStart,
    VisitMediaTypesEnd,
    VisitLinksStart,
    VisitLinksEnd,
    VisitAsyncCallbacksStart,
    VisitAsyncCallbacksEnd,
    VisitAsyncCallbackReferenceStart,
    VisitAsyncCallbackReferenceEnd,
    VisitAsyncCallbackStart,
    VisitAsyncCallbackEnd,
    VisitHeadersStart,
    VisitHeadersEnd,
    VisitSecuritySchemeReferenceStart,
    VisitSecuritySchemeReferenceEnd,
    VisitSecuritySchemesStart,
    VisitSecuritySchemesEnd,
    VisitHeaderReferenceStart,
    VisitHeaderReferenceEnd,
    VisitHeaderStart,
    VisitHeaderEnd,
    VisitRequestBodyReferenceStart,
    VisitRequestBodyReferenceEnd,
    VisitRequestBodyStart,
    VisitRequestBodyEnd,
    VisitExampleReferenceStart,
    VisitExampleReferenceEnd,
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
    VisitLinkReferenceStart,
    VisitLinkReferenceEnd,
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
    VisitSchemaReferenceStart,
    VisitSchemaReferenceEnd,
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
    VisitObjectPropertyReferenceStart,
    VisitObjectPropertyReferenceEnd,
    VisitObjectPropertyStart,
    VisitObjectPropertyEnd,
    VisitObjectPropertiesStart,
    VisitObjectPropertiesEnd,
    VisitObjectEnd,
    VisitAnySchemaStart,
    VisitAnySchemaEnd,
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
        call_id: Option<&str>,
        out_path: &Path,
        args: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        let lua_vm = get_lua_vm();
        let func = scripts::get_lua_function(self, &lua_vm)?;

        let args_value = serde_json::to_value(args)?;

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

        let call_id_value = lua_vm.to_value(&call_id);

        let code: mlua::Value = match &args_vec {
            args if args.len() == 1 => func.call((&args[0], call_id_value)),
            args if args.len() == 2 => func.call((&args[0], &args[1], call_id_value)),
            args if args.len() == 3 => func.call((&args[0], &args[1], &args[2], call_id_value)),
            args if args.len() == 4 => {
                func.call((&args[0], &args[1], &args[2], &args[3], call_id_value))
            }
            args if args.len() == 5 => func.call((
                &args[0],
                &args[1],
                &args[2],
                &args[3],
                &args[4],
                call_id_value,
            )),
            args if args.len() == 6 => func.call((
                &args[0],
                &args[1],
                &args[2],
                &args[3],
                &args[4],
                &args[5],
                call_id_value,
            )),
            args if args.len() == 7 => func.call((
                &args[0],
                &args[1],
                &args[2],
                &args[3],
                &args[4],
                &args[5],
                &args[6],
                call_id_value,
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
                call_id_value,
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
                call_id_value,
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
                call_id_value,
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
                "Failed to call lua script {} with args {:?}",
                self, args_value
            )
        })?;

        code::save_code(out_path, lua_vm.from_value(code)?)?;

        Ok(())
    }

    pub fn call_func(&self, call_id: Option<&str>) -> Result<()> {
        let lua_vm = get_lua_vm();
        let func = scripts::get_lua_function(self, &lua_vm)?;
        func.call::<_, ()>(lua_vm.to_value(&call_id))
            .with_context(|| format!("Could not call lua function [{}]", self))?;
        Ok(())
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
            Script::Target => r#"return require("target")"#,
            Script::VisitResponsesStart => "return VISITORS.components.responses.visitResponsesStart",
            Script::VisitResponsesEnd => "return VISITORS.components.responses.visitResponsesEnd",
            Script::VisitResponseStart => "return VISITORS.components.responses.visitResponseStart",
            Script::VisitResponseEnd => "return VISITORS.components.responses.visitResponseEnd",
            Script::VisitHeadersStart => "return VISITORS.common.headers.visitHeadersStart",
            Script::VisitHeadersEnd => "return VISITORS.common.headers.visitHeadersEnd",
            Script::VisitHeaderStart => "return VISITORS.common.headers.header.visitHeaderStart",
            Script::VisitHeaderEnd => "return VISITORS.common.headers.header.visitHeaderEnd",
            Script::VisitExamplesStart => "return VISITORS.common.examples.visitExamplesStart",
            Script::VisitExampleStart => "return VISITORS.common.examples.visitExampleStart",
            Script::VisitExampleEnd => "return VISITORS.common.examples.visitExampleEnd",
            Script::VisitExamplesEnd => "return VISITORS.common.examples.visitExamplesEnd",
            Script::VisitParameterSchemaOrContentStart => {
                "return VISITORS.common.parameter_schema_or_content.visitParameterSchemaOrContentStart"
            }
            Script::VisitParameterSchemaOrContentEnd => {
                "return VISITORS.common.parameter_schema_or_content.visitParameterSchemaOrContentEnd"
            }
            Script::VisitMediaTypeStart => "return VISITORS.common.media_types.media_type.visitMediaTypeStart",
            Script::VisitMediaTypeEnd => "return VISITORS.common.media_types.media_type.visitMediaTypeEnd",
            Script::VisitEncodingStart => {
                "return VISITORS.common.media_types.media_type.encoding.visitEncodingStart"
            }
            Script::VisitEncodingEnd => "return VISITORS.common.media_types.media_type.encoding.visitEncodingEnd",
            Script::VisitSchemasStart => "return VISITORS.components.schemas.visitSchemasStart",
            Script::VisitSchemasEnd => "return VISITORS.components.schemas.visitSchemasEnd",
            Script::VisitSchemaStart => "return VISITORS.components.schemas.visitSchemaStart",
            Script::VisitSchemaEnd => "return VISITORS.components.schemas.visitSchemaEnd",
            Script::VisitDefault => "return VISITORS.components.schemas.visitDefault",
            Script::VisitDiscriminator => "return VISITORS.components.schemas.visitDiscriminator",
            Script::VisitSpecStart => "return VISITORS.visitSpecStart",
            Script::VisitSpecEnd => "return VISITORS.visitSpecEnd",
            Script::VisitExternalDocs => "return VISITORS.common.external_docs.visitExternalDocs",
            Script::VisitSpecTag => "return VISITORS.tags.visitSpecTag",
            Script::VisitSpecTagsStart => "return VISITORS.tags.visitSpecTagsStart",
            Script::VisitSpecTagsEnd => "return VISITORS.tags.visitSpecTagsEnd",
            Script::VisitServersStart => "return VISITORS.common.servers.visitServersStart",
            Script::VisitServersEnd => "return VISITORS.common.servers.visitServersEnd",
            Script::VisitServerStart => "return VISITORS.common.servers.server.visitServerStart",
            Script::VisitServerEnd => "return VISITORS.common.servers.server.visitServerEnd",
            Script::VisitServerVariable => "return VISITORS.common.servers.server.visitServerVariable",
            Script::VisitSpecInfoStart => "return VISITORS.info.visitSpecInfoStart",
            Script::VisitSpecInfoEnd => "return VISITORS.info.visitSpecInfoEnd",
            Script::VisitSpecInfoContact => "return VISITORS.info.visitSpecInfoContact",
            Script::VisitSpecInfoLicense => "return VISITORS.info.visitSpecInfoLicense",
            Script::VisitSecurityRequirementsStart => {
                "return VISITORS.common.security_requirements.visitSecurityRequirementsStart"
            }
            Script::VisitSecurityRequirementsEnd => {
                "return VISITORS.common.security_requirements.visitSecurityRequirementsEnd"
            }
            Script::VisitSecurityRequirement => {
                "return VISITORS.common.security_requirements.visitSecurityRequirement"
            }
            Script::VisitObjectStart => "return VISITORS.components.schemas.kind.type.object.visitObjectStart",
            Script::VisitObjectEnd => "return VISITORS.components.schemas.kind.type.object.visitObjectEnd",
            Script::VisitAnySchemaStart => "return VISITORS.components.schemas.kind.any.visitAnySchemaStart",
            Script::VisitAnySchemaEnd => "return VISITORS.components.schemas.kind.any.visitAnySchemaEnd",
            Script::VisitPropertyNotStart => "return VISITORS.components.schemas.kind.not.visitPropertyNotStart",
            Script::VisitPropertyNotEnd => "return VISITORS.components.schemas.kind.not.visitPropertyNotEnd",
            Script::VisitAdditionalPropertiesAny => {
                "return VISITORS.components.schemas.kind.type.object.visitAdditionalPropertiesAny"
            }
            Script::VisitStringProperty => {
                "return VISITORS.components.schemas.kind.type.string.visitStringProperty"
            }
            Script::VisitNumberProperty => {
                "return VISITORS.components.schemas.kind.type.number.visitNumberProperty"
            }
            Script::VisitIntegerProperty => {
                "return VISITORS.components.schemas.kind.type.integer.visitIntegerProperty"
            }
            Script::VisitArrayPropertyStart => {
                "return VISITORS.components.schemas.kind.type.object.visitArrayPropertyStart"
            }
            Script::VisitArrayPropertyEnd => {
                "return VISITORS.components.schemas.kind.type.object.visitArrayPropertyEnd"
            }
            Script::VisitBooleanProperty => {
                "return VISITORS.components.schemas.kind.type.boolean.visitBooleanProperty"
            }
            Script::VisitOneOfStart => "return VISITORS.components.schemas.kind.oneOf.visitOneOfStart",
            Script::VisitOneOfEnd => "return VISITORS.components.schemas.kind.oneOf.visitOneOfEnd",
            Script::VisitAllOfStart => "return VISITORS.components.schemas.kind.allOf.visitAllOfStart",
            Script::VisitAllOfEnd => "return VISITORS.components.schemas.kind.allOf.visitAllOfEnd",
            Script::VisitAnyOfStart => "return VISITORS.components.schemas.kind.anyOf.visitAnyOfStart",
            Script::VisitAnyOfEnd => "return VISITORS.components.schemas.kind.anyOf.visitAnyOfEnd",
            Script::VisitAdditionalPropertiesStart => {
                "return VISITORS.components.schemas.kind.type.object.visitAdditionalPropertiesStart"
            }
            Script::VisitAdditionalPropertiesEnd => {
                "return VISITORS.components.schemas.kind.type.object.visitAdditionalPropertiesEnd"
            }
            Script::VisitObjectPropertyStart => {
                "return VISITORS.components.schemas.kind.type.object.visitObjectPropertyStart"
            }
            Script::VisitObjectPropertyEnd => {
                "return VISITORS.components.schemas.kind.type.object.visitObjectPropertyEnd"
            }
            Script::VisitGenericExample => "return VISITORS.common.generic_example.visitGenericExample",
            Script::VisitEncodingsStart => {
                "return VISITORS.common.media_types.media_type.visitMediaTypeEncodingsStart"
            }
            Script::VisitEncodingsEnd => "return VISITORS.common.media_types.media_type.visitMediaTypeEncodingsEnd",
            Script::VisitObjectPropertiesStart => {
                "return VISITORS.components.schemas.kind.type.object.visitObjectPropertiesStart"
            }
            Script::VisitObjectPropertiesEnd => {
                "return VISITORS.components.schemas.kind.type.object.visitObjectPropertiesEnd"
            }
            Script::VisitMediaTypesStart => "return VISITORS.common.media_types.visitMediaTypesStart",
            Script::VisitMediaTypesEnd => "return VISITORS.common.media_types.visitMediaTypesEnd",
            Script::VisitLinkStart => "return VISITORS.common.links.link.visitLinkStart",
            Script::VisitLinkEnd => "return VISITORS.common.links.link.visitLinkEnd",
            Script::VisitGenericRequestBody => {
                "return VISITORS.common.generic_request_body.visitGenericRequestBody"
            }
            Script::VisitGenericParametersStart => {
                "return VISITORS.common.generic_parameters.visitGenericParametersStart"
            }
            Script::VisitGenericParametersEnd => {
                "return VISITORS.common.generic_parameters.visitGenericParametersEnd"
            }
            Script::VisitGenericParameter => "return VISITORS.common.generic_parameters.visitGenericParameter",
            Script::VisitLinksStart => "return VISITORS.common.links.visitLinksStart",
            Script::VisitLinksEnd => "return VISITORS.common.links.visitLinksEnd",
            Script::VisitParametersStart => "return VISITORS.common.parameters.visitParametersStart",
            Script::VisitParametersEnd => "return VISITORS.common.parameters.visitParametersEnd",
            Script::VisitQueryParameterStart => {
                "return VISITORS.common.parameters.query_parameter.visitQueryParameterStart"
            }
            Script::VisitQueryParameterEnd => {
                "return VISITORS.common.parameters.query_parameter.visitQueryParameterEnd"
            }
            Script::VisitParameterDataStart => "return VISITORS.common.parameter_data.visitParameterDataStart",
            Script::VisitParameterDataEnd => "return VISITORS.common.parameter_data.visitParameterDataEnd",
            Script::VisitHeaderParameterStart => {
                "return VISITORS.common.parameters.header_parameter.visitHeaderParameterStart"
            }
            Script::VisitHeaderParameterEnd => {
                "return VISITORS.common.parameters.header_parameter.visitHeaderParameterEnd"
            }
            Script::VisitPathParameterStart => {
                "return VISITORS.common.parameters.path_parameter.visitPathParameterStart"
            }
            Script::VisitPathParameterEnd => {
                "return VISITORS.common.parameters.path_parameter.visitPathParameterEnd"
            }
            Script::VisitCookieParameterStart => {
                "return VISITORS.common.parameters.cookie_parameter.visitCookieParameterStart"
            }
            Script::VisitCookieParameterEnd => {
                "return VISITORS.common.parameters.cookie_parameter.visitCookieParameterEnd"
            }
            Script::VisitRequestBodiesStart => "return VISITORS.components.request_bodies.visitRequestBodiesStart",
            Script::VisitRequestBodiesEnd => "return VISITORS.components.request_bodies.visitRequestBodiesEnd",
            Script::VisitRequestBodyStart => "return VISITORS.components.request_bodies.visitRequestBodyStart",
            Script::VisitRequestBodyEnd => "return VISITORS.components.request_bodies.visitRequestBodyEnd",
            Script::VisitSecuritySchemesStart => {
                "return VISITORS.components.security_schemes.visitSecuritySchemesStart"
            }
            Script::VisitSecuritySchemesEnd => {
                "return VISITORS.components.security_schemes.visitSecuritySchemesEnd"
            }
            Script::VisitSecuritySchemeApiKey => {
                "return VISITORS.components.security_schemes.visitSecuritySchemeApiKey"
            }
            Script::VisitSecuritySchemeHttp => {
                "return VISITORS.components.security_schemes.visitSecuritySchemeHttp"
            }
            Script::VisitSecuritySchemeOAuth2Start => {
                "return VISITORS.components.security_schemes.visitSecuritySchemeOAuth2Start"
            }
            Script::VisitSecuritySchemeOAuth2End => {
                "return VISITORS.components.security_schemes.visitSecuritySchemeOAuth2End"
            }
            Script::VisitSecuritySchemeOAuth2FlowsStart => {
                "return VISITORS.components.security_schemes.visitSecuritySchemeOAuth2FlowsStart"
            }
            Script::VisitSecuritySchemeOAuth2FlowsEnd => {
                "return VISITORS.components.security_schemes.visitSecuritySchemeOAuth2FlowsEnd"
            }
            Script::VisitSecuritySchemeOAuth2FlowImplicit => {
                "return VISITORS.components.security_schemes.visitSecuritySchemeOAuth2FlowImplicit"
            }
            Script::VisitSecuritySchemeOAuth2FlowPassword => {
                "return VISITORS.components.security_schemes.visitSecuritySchemeOAuth2FlowPassword"
            }
            Script::VisitSecuritySchemeOAuth2FlowClientCredentials => {
                "return VISITORS.components.security_schemes.visitSecuritySchemeOAuth2FlowClientCredentials"
            }
            Script::VisitSecuritySchemeOAuth2FlowAuthorizationCode => {
                "return VISITORS.components.security_schemes.visitSecuritySchemeOAuth2FlowAuthorizationCode"
            }
            Script::VisitSecuritySchemeOpenIdConnect => {
                "return VISITORS.components.security_schemes.visitSecuritySchemeOpenIdConnect"
            }
            Script::VisitComponentsStart => "return VISITORS.components.visitComponentsStart",
            Script::VisitComponentsEnd => "return VISITORS.components.visitComponentsEnd",
            Script::VisitAsyncCallbacksStart => {
                "return VISITORS.components.async_callbacks.visitAsyncCallbacksStart"
            }
            Script::VisitAsyncCallbacksEnd => "return VISITORS.components.async_callbacks.visitAsyncCallbacksEnd",
            Script::VisitAsyncCallbackStart => "return VISITORS.common.async_callback.visitAsyncCallbackStart",
            Script::VisitAsyncCallbackEnd => "return VISITORS.common.async_callback.visitAsyncCallbackEnd",
            Script::VisitPathItemStart => "return VISITORS.common.path_item.visitPathItemStart",
            Script::VisitPathItemEnd => "return VISITORS.common.path_item.visitPathItemEnd",
            Script::VisitOperationResponsesStart => {
                "return VISITORS.common.operation.responses.visitOperationResponsesStart"
            }
            Script::VisitOperationResponsesEnd => {
                "return VISITORS.common.operation.responses.visitOperationResponsesEnd"
            }
            Script::VisitTraceOperationStart => "return VISITORS.common.operation.visitTraceOperationStart",
            Script::VisitTraceOperationEnd => "return VISITORS.common.operation.visitTraceOperationEnd",
            Script::VisitPutOperationStart => "return VISITORS.common.operation.visitPutOperationStart",
            Script::VisitPutOperationEnd => "return VISITORS.common.operation.visitPutOperationEnd",
            Script::VisitPostOperationStart => "return VISITORS.common.operation.visitPostOperationStart",
            Script::VisitPostOperationEnd => "return VISITORS.common.operation.visitPostOperationEnd",
            Script::VisitPatchOperationStart => "return VISITORS.common.operation.visitPatchOperationStart",
            Script::VisitPatchOperationEnd => "return VISITORS.common.operation.visitPatchOperationEnd",
            Script::VisitOptionsOperationStart => "return VISITORS.common.operation.visitOptionsOperationStart",
            Script::VisitOptionsOperationEnd => "return VISITORS.common.operation.visitOptionsOperationEnd",
            Script::VisitHeadOperationStart => "return VISITORS.common.operation.visitHeadOperationStart",
            Script::VisitHeadOperationEnd => "return VISITORS.common.operation.visitHeadOperationEnd",
            Script::VisitGetOperationStart => "return VISITORS.common.operation.visitGetOperationStart",
            Script::VisitGetOperationEnd => "return VISITORS.common.operation.visitGetOperationEnd",
            Script::VisitDeleteOperationStart => "return VISITORS.common.operation.visitDeleteOperationStart",
            Script::VisitDeleteOperationEnd => "return VISITORS.common.operation.visitDeleteOperationEnd",
            Script::VisitPathsStart => "return VISITORS.paths.visitPathsStart",
            Script::VisitPathsEnd => "return VISITORS.paths.visitPathsEnd",
            //References
            Script::VisitResponseReferenceStart => "return VISITORS.components.responses.visitResponseReferenceStart",
            Script::VisitResponseReferenceEnd => "return VISITORS.components.responses.visitResponseReferenceEnd",
            Script::VisitSchemaReferenceStart => "return VISITORS.components.schemas.visitSchemaReferenceStart",
            Script::VisitSchemaReferenceEnd => "return VISITORS.components.schemas.visitSchemaReferenceEnd",
            Script::VisitExampleReferenceStart => "return VISITORS.common.examples.visitExampleReferenceStart",
            Script::VisitExampleReferenceEnd => "return VISITORS.common.examples.visitExampleReferenceEnd",
            Script::VisitRequestBodyReferenceStart => {
                "return VISITORS.components.request_bodies.visitRequestBodyReferenceStart"
            }
            Script::VisitRequestBodyReferenceEnd => {
                "return VISITORS.components.request_bodies.visitRequestBodyReferenceEnd"
            }
            Script::VisitLinkReferenceStart => "return VISITORS.common.links.link.visitLinkReferenceStart",
            Script::VisitLinkReferenceEnd => "return VISITORS.common.links.link.visitLinkReferenceEnd",
            Script::VisitAsyncCallbackReferenceStart => {
                "return VISITORS.common.async_callback.visitAsyncCallbackReferenceStart"
            }
            Script::VisitAsyncCallbackReferenceEnd => {
                "return VISITORS.common.async_callback.visitAsyncCallbackReferenceEnd"
            }
            Script::VisitHeaderReferenceStart => "return VISITORS.common.headers.header.visitHeaderReferenceStart",
            Script::VisitHeaderReferenceEnd => "return VISITORS.common.headers.header.visitHeaderReferenceEnd",
            Script::VisitSecuritySchemeReferenceStart => {
                "return VISITORS.components.security_schemes.visitSecuritySchemeReferenceStart"
            }
            Script::VisitSecuritySchemeReferenceEnd => {
                "return VISITORS.components.security_schemes.visitSecuritySchemeReferenceEnd"
            }
            Script::VisitPathItemReferenceStart => "return VISITORS.common.path_item.visitPathItemReferenceStart",
            Script::VisitPathItemReferenceEnd => "return VISITORS.common.path_item.visitPathItemReferenceEnd",
            Script::VisitParameterReferenceStart => "return VISITORS.common.parameters.visitParameterReferenceStart",
            Script::VisitParameterReferenceEnd => "return VISITORS.common.parameters.visitParameterReferenceEnd",
            Script::VisitObjectPropertyReferenceStart => {
                "return VISITORS.components.schemas.kind.type.object.visitObjectPropertyReferenceStart"
            },
            Script::VisitObjectPropertyReferenceEnd => {
                "return VISITORS.components.schemas.kind.type.object.visitObjectPropertyReferenceEnd"
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
