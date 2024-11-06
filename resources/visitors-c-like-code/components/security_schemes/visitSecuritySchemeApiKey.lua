--- This visitor is invoked for processing API key security scheme
--- @param securitySchemeName string|null #
--- @param securityScheme APIKeySecurityScheme #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSecuritySchemeApiKey(securitySchemeName, securityScheme, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSecuritySchemeApiKey", visitSecuritySchemeApiKey)
