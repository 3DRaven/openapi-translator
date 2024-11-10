--- This visitor is invoked for processing OAuth2 security scheme client credentials flow
--- @param flow ClientCredentialsOAuth2Flow #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecuritySchemeOAuth2FlowClientCredentials(flow, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSecuritySchemeOAuth2FlowClientCredentials",
visitSecuritySchemeOAuth2FlowClientCredentials)
