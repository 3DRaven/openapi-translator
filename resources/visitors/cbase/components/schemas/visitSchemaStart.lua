--- This visitor is invoked before processing any kind of schema
--- @param schemaName string|null #
--- @param schemaDescriptor Schema # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSchemaStart(schemaName, schemaDescriptor, extensions, callId)
    --- At this point, there might not be a name set
    --- Variants:
    --- 1. additionalProperties, can be object, primitive, $ref
    ---    schemaName is NULL, reference script may be called after visitAdditionalPropertiesStart script
    ---    if reference script called, name already replced in global stack from reference
    ---    if reference script does't called, name is "collected names .. AdditionalProperties"
    ---    it set by visitAdditionalPropertiesStart script
    --- 2. schema without reference
    ---     name in schemaName, schemaReference script not called
    --- 3. schema reference
    ---     schemaName is NULL, schemaReference script called
    if nullableAsNillable(schemaName) ~= nil then
        GLOBAL_CONTEXT.names:push(extensions[Extensions.MODEL_NAME] or schemaName)
    end
    return {}
end

return functionCallAndLog("visitSchemaStart", visitSchemaStart)
