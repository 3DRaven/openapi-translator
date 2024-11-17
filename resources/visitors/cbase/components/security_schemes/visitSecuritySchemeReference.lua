--- This visitor is invoked for processing security scheme if reference
--- @param securitySchemeName string|null #
--- @param securitySchemeReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecuritySchemeReference(securitySchemeName, securitySchemeReference, extensions, callId)
    return {}
end

return functionCallAndLog("visitSecuritySchemeReference", visitSecuritySchemeReference)