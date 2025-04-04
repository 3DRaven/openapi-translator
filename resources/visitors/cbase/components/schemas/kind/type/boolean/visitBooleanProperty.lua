--- This visitor is invoked when a property of type boolean is found.
--- Returns a code for creating storage for additionalProperties (Map as example)
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param booleanDescriptor BooleanType # boolean property descriptor
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitBooleanProperty(booleanDescriptor, extensions, callId)
    return {}
end

return functionCallAndLog("visitBooleanProperty", visitBooleanProperty)
