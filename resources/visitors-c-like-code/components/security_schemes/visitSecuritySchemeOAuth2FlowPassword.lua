--- This visitor is invoked for processing OAuth2 password flow
--- @param flow PasswordOAuth2Flow #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecuritySchemeOAuth2FlowPassword(flow, extensions, callId)
    return {}
end

return functionCallAndLog("visitSecuritySchemeOAuth2FlowPassword", visitSecuritySchemeOAuth2FlowPassword)
