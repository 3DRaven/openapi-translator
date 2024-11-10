--- This visitor is invoked before processing security schemes
--- @param securitySchemes table<string,ReferenceOr<table>> # generic collection of security schemes
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecuritySchemesStart(securitySchemes, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSecuritySchemesStart", visitSecuritySchemesStart)
