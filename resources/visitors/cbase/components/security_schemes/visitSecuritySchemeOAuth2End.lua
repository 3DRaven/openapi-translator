--- This visitor is invoked after processing OAuth2 security scheme
--- @param securitySchemeName string|null #
--- @param securityScheme OAuth2SecurityScheme #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecuritySchemeOAuth2End(securitySchemeName, securityScheme, extensions, callId)
    return {}
end

return functionCallAndLog("visitSecuritySchemeOAuth2End", visitSecuritySchemeOAuth2End)
