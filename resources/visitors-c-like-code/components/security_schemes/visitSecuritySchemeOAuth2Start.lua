--- This visitor is invoked before processing OAuth2 security scheme
--- @param securitySchemeName string|null #
--- @param securityScheme OAuth2SecurityScheme #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSecuritySchemeOAuth2Start(securitySchemeName, securityScheme, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSecuritySchemeOAuth2Start", visitSecuritySchemeOAuth2Start)
