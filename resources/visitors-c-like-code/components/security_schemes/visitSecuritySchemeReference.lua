--- This visitor is invoked for processing security scheme if reference
--- @param securitySchemeName string|null #
--- @param securitySchemeReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecuritySchemeReference(securitySchemeName, securitySchemeReference, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSecuritySchemeReference", visitSecuritySchemeReference)
