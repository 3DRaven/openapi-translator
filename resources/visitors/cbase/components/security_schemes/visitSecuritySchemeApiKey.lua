--- This visitor is invoked for processing API key security scheme
--- @param securitySchemeName string|null #
--- @param securityScheme APIKeySecurityScheme #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecuritySchemeApiKey(securitySchemeName, securityScheme, extensions, callId)
    return {}
end

return functionCallAndLog("visitSecuritySchemeApiKey", visitSecuritySchemeApiKey)
