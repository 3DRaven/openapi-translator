--- This visitor is invoked for processing OAuth2 security scheme authorization code flow
--- @param flow AuthorizationCodeOAuth2Flow #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecuritySchemeOAuth2FlowAuthorizationCode(flow, extensions, callId)
    return {}
end

return functionCallAndLog("visitSecuritySchemeOAuth2FlowAuthorizationCode",
    visitSecuritySchemeOAuth2FlowAuthorizationCode)
