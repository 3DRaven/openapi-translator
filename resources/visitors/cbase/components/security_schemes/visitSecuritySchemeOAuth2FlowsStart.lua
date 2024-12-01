--- This visitor is invoked before processing OAuth2 flows
--- @param flows OAuth2Flows #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecuritySchemeOAuth2FlowsStart(flows, extensions, callId)
    return {}
end

return functionCallAndLog("visitSecuritySchemeOAuth2FlowsStart", visitSecuritySchemeOAuth2FlowsStart, 1)
