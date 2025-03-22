--- This visitor is invoked to complete processing a schema containing an array, regardless of whether
--- the schema is nested within an object or is a top-level schema. The invocation occurs after processing
--- the schemas contained within the array.
--- Returns a code for array model end
--- @param arrayDescriptor ArrayType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitArrayPropertyEnd(arrayDescriptor, extensions, callId)
    return {}
end

return functionCallAndLog("visitArrayPropertyEnd", visitArrayPropertyEnd, -1)
