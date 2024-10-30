--- This visitor is invoked after processing OAuth2 security scheme
--- @param securitySchemeName string #
--- @param securityScheme OAuth2SecurityScheme #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSecuritySchemeOAuth2End(securitySchemeName, securityScheme, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSecuritySchemeOAuth2End", visitSecuritySchemeOAuth2End)