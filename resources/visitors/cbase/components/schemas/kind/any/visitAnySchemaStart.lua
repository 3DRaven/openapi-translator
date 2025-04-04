--- This visitor is invoked to process a found schema without a defined structure
--- such as `additionalProperties: {}`.
--- Returns a model name with unknown structure (it is {} in OpenAPI)
--- @param anySchemaDescriptor AnySchema # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitAnySchemaStart(anySchemaDescriptor, extensions, callId)
    return {}
end

return functionCallAndLog("visitAnySchemaStart", visitAnySchemaStart, 1)
