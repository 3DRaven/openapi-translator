--- This visitor is invoked after processing any kind of schema if reference
--- @param schemaName string|null #
--- @param schemaReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSchemaReferenceEnd(schemaName, schemaReference, extensions, callId)
    -- if we found reference we every time use names from reference or from extension
    GLOBAL_CONTEXT.names:clear()
    GLOBAL_CONTEXT.names:pushAll(GLOBAL_CONTEXT.savedNames.items)
    return {}
end

return functionCallAndLog("visitSchemaReferenceEnd", visitSchemaReferenceEnd)
