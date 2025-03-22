--- This visitor handles the processing before property of object schema.
--- @param propertyName string|null #
--- @param schema Schema # free form of additionalProperties has this value
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitObjectPropertyStart(propertyName, schema, extensions, callId)
    GLOBAL_CONTEXT.names:push(propertyName)
    return {}
end

return functionCallAndLog("visitObjectPropertyStart", visitObjectPropertyStart, 1)
