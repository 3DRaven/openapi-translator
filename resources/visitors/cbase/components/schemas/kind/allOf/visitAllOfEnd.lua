--- This visitor is invoked after processing allOf schemas
--- @param schemas ReferenceOr<Schema>[] #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitAllOfEnd(schemas, extensions, callId)
    return {}
end

return functionCallAndLog("visitAllOfEnd", visitAllOfEnd, -1)
