--- This visitor is invoked before processing any kind of schema if reference
--- @param schemaName string|null #
--- @param schemaReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSchemaReferenceStart(schemaName, schemaReference, extensions, callId)
    -- if we found reference we every time use names from reference or from extension
    GLOBAL_CONTEXT.savedNames:pushAll(GLOBAL_CONTEXT.names.items)
    GLOBAL_CONTEXT.names:clear()
    GLOBAL_CONTEXT.names:push(extensions[Extensions.MODEL_NAME] or lastReferencePart(schemaReference))
    return {}
end

return functionCallAndLog("visitSchemaReferenceStart", visitSchemaReferenceStart)
