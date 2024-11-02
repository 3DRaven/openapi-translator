--- This visitor is invoked before processing any kind of schema
--- @param schemaName string|null #
--- @param schemaDescriptor Schema # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSchemaStart(schemaName, schemaDescriptor, extensions, callsStack)
    -- At this point, there might not be a name, for example, for additionalProperties
    local name = getFirstExistsName(extensions[Extensions.MODEL_NAME], schemaName)
    if name then
        GLOBAL_CONTEXT.names:push(name)
    else
        print("Name is empty, skip")
    end
    return {}
end

return functionCallAndLog("visitSchemaStart", visitSchemaStart)
