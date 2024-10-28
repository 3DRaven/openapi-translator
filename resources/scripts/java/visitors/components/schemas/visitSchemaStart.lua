--- This visitor is invoked before processing any kind of schema
--- @param schemaName string|nil #
--- @param schemaDescriptor Schema # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSchemaStart(schemaName, schemaDescriptor, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSchemaStart", visitSchemaStart)
