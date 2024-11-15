--- This visitor handles the processing of object schema property.
--- @param propertyName string|null #
--- @param schema Schema # free form of additionalProperties has this value
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitObjectPropertyEnd(propertyName, schema, extensions, callId)
    return {}
end

return functionCallAndLog("visitObjectPropertyEnd", visitObjectPropertyEnd)
