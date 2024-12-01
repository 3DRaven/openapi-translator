--- This visitor is invoked before processing anyOf schema
--- @param schemas ReferenceOr<Schema>[] #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitAnyOfElementStart(schemas, extensions, callId)
    return {}
end

return functionCallAndLog("visitAnyOfElementStart", visitAnyOfElementStart, 1)
