--- This visitor is invoked before processing security scheme if reference
--- @param securitySchemeName string|null #
--- @param securitySchemeReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecuritySchemeReferenceStart(securitySchemeName, securitySchemeReference, extensions, callId)
    return {}
end

return functionCallAndLog("visitSecuritySchemeReferenceStart", visitSecuritySchemeReferenceStart, 1)
