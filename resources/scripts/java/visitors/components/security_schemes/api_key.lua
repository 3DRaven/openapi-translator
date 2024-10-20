--- This visitor is invoked for processing API key security scheme
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param securityScheme APIKeySecurityScheme #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSecuritySchemeApiKey(namesStack, securityScheme, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSecuritySchemeApiKey", visitSecuritySchemeApiKey)
