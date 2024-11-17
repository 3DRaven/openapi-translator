--- This visitor is invoked for processing HTTP security scheme
--- @param securitySchemeName string|null #
--- @param securityScheme HTTPSecurityScheme #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecuritySchemeHttp(securitySchemeName, securityScheme, extensions, callId)
    return {}
end

return functionCallAndLog("visitSecuritySchemeHttp", visitSecuritySchemeHttp)
