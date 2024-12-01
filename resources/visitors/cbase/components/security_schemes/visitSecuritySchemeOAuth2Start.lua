--- This visitor is invoked before processing OAuth2 security scheme
--- @param securitySchemeName string|null #
--- @param securityScheme OAuth2SecurityScheme #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecuritySchemeOAuth2Start(securitySchemeName, securityScheme, extensions, callId)
    return {}
end

return functionCallAndLog("visitSecuritySchemeOAuth2Start", visitSecuritySchemeOAuth2Start, 1)
