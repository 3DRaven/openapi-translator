--- This visitor is invoked before processing oneOf schemas
--- @param schemas ReferenceOr<Schema>[] #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitOneOfStart(schemas, extensions, callId)
    GLOBAL_CONTEXT.models:push(OneOfModel.new("stub-oneOf-model-name"))
    return {}
end

return functionCallAndLog("visitOneOfStart", visitOneOfStart)
