--- This visitor is invoked after all the content inside a schema of type object has been processed.
--- Returns a code of the end of an object based on whether it's required.
--- @param objectDescriptor ObjectType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitObjectEnd(objectDescriptor, extensions, callId)
    return {}
end

return functionCallAndLog("visitObjectEnd", visitObjectEnd, -1)
