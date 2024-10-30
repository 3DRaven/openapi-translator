--- This visitor is invoked after processing any kind of schema
--- @param schemaName string|null #
--- @param schemaDescriptor Schema # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSchemaEnd(schemaName, schemaDescriptor, extensions, callsStack)
    local name = getName(extensions[Extensions.MODEL_NAME], schemaName)
    if name then
        global_context.names:pop()
    else
        print("Name is empty")
    end
    return {}
end

return functionCallAndLog("visitSchemaEnd", visitSchemaEnd)
