--- This visitor is called after the schema in the not property is processed
--- @param schema ReferenceOr<Schema>
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitPropertyNotEnd(schema, namesStack, extensions, callId)
    return {}
end

return functionCallAndLog("visitPropertyNotEnd", visitPropertyNotEnd, -1)
