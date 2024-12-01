--- This visitor is invoked after processing OAuth2 flows
--- @param flows OAuth2Flows #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecuritySchemeOAuth2FlowsEnd(flows, extensions, callId)
    return {}
end

return functionCallAndLog("visitSecuritySchemeOAuth2FlowsEnd", visitSecuritySchemeOAuth2FlowsEnd, -1)
