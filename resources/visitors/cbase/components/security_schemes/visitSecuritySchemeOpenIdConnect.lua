--- This visitor is invoked for processing OAuth2 security scheme OpenID Connect flow
--- @param securitySchemeName string|null
--- @param securityScheme OpenIDConnectSecurityScheme #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecuritySchemeOpenIdConnect(securitySchemeName, securityScheme, extensions, callId)
    return {}
end

return functionCallAndLog("visitSecuritySchemeOpenIdConnect", visitSecuritySchemeOpenIdConnect)
