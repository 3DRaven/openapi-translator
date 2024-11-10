--- This visitor is invoked for processing OAuth2 security scheme OpenID Connect flow
--- @param securitySchemeName string|null
--- @param securityScheme OpenIDConnectSecurityScheme #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecuritySchemeOpenIdConnect(securitySchemeName, securityScheme, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSecuritySchemeOpenIdConnect", visitSecuritySchemeOpenIdConnect)
