--- This visitor handles the processing before properties of object schema.
--- @param properties table<string,ReferenceOr<Schema>> #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitObjectPropertiesStart(properties, extensions, callId)
    return {}
end

return functionCallAndLog("visitObjectPropertiesStart", visitObjectPropertiesStart, 1)
