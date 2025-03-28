--- This visitor is invoked when a property of type number is found.
--- Returns a code for creating property for number value
--- @param numberDescriptor StringType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitNumberProperty(numberDescriptor, extensions, callId)
    return {}
end

return functionCallAndLog("visitNumberProperty", visitNumberProperty)
