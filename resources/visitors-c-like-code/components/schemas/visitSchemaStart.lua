--- This visitor is invoked before processing any kind of schema
--- @param schemaName string|null #
--- @param schemaDescriptor Schema # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSchemaStart(schemaName, schemaDescriptor, extensions, callsStack)
    -- At this point, there might not be a name, for example, for additionalProperties
    -- If schemaName is null, it is reference and name already set
    if nullableAsNillable(schemaName) ~= nil then
        local name = extensions[Extensions.MODEL_NAME] or nullableAsNillable(schemaName)
        if name then
            GLOBAL_CONTEXT.names:push(name)
        else
            -- if name is unknown then it is additionalProperties
            -- we set here dummy-name only for deleting it at visitSchemaEnd
            GLOBAL_CONTEXT.names:push("dummy-name")
        end
    end
    return {}
end

return functionCallAndLog("visitSchemaStart", visitSchemaStart)
