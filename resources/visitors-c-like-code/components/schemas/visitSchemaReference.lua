--- This visitor is invoked before processing any kind of schema if reference
--- @param schemaName string|null #
--- @param schemaReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSchemaReference(schemaName, schemaReference, extensions, callsStack)
    -- if it called then we can set name
    GLOBAL_CONTEXT.names:push(extensions[Extensions.MODEL_NAME] or lastReferencePart(schemaReference))
    return {}
end

return functionCallAndLog("visitSchemaReference", visitSchemaReference)
