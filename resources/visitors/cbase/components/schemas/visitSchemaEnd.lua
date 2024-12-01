--- This visitor is invoked after processing any kind of schema
--- @param schemaName string|null #
--- @param schemaDescriptor Schema # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSchemaEnd(schemaName, schemaDescriptor, extensions, callId)
    -- see visitSchemaStart script to comments
    if nullableAsNillable(schemaName) ~= nil then
        GLOBAL_CONTEXT.names:pop()
    end
    return {}
end

return functionCallAndLog("visitSchemaEnd", visitSchemaEnd, -1)
