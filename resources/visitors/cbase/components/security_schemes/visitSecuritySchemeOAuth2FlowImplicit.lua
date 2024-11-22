--- This visitor is invoked for processing OAuth2 security scheme implicit flow
--- @param flow ImplicitOAuth2Flow #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecuritySchemeOAuth2FlowImplicit(flow, extensions, callId)
    return {}
end

return functionCallAndLog("visitSecuritySchemeOAuth2FlowImplicit", visitSecuritySchemeOAuth2FlowImplicit)
