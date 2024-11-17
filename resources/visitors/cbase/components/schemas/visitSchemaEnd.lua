--- This visitor is invoked after processing any kind of schema
--- @param schemaName string|null #
--- @param schemaDescriptor Schema # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSchemaEnd(schemaName, schemaDescriptor, extensions, callId)
    GLOBAL_CONTEXT.names:pop()
    return {}
end

return functionCallAndLog("visitSchemaEnd", visitSchemaEnd)
