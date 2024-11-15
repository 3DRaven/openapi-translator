--- This visitor handles the processing after object schema properties.
--- @param properties table<string,ReferenceOr<Schema>> #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitObjectPropertiesEnd(properties, extensions, callId)
    return {}
end

return functionCallAndLog("visitObjectPropertiesEnd", visitObjectPropertiesEnd)
