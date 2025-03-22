--- This visitor is invoked to start processing a schema containing an array, regardless of whether the schema
--- is nested within an object or is a top-level schema. The invocation occurs before processing the schemas
--- contained within the array.
--- Returns a code of start for array property of object
--- @param arrayDescriptor ArrayType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitArrayPropertyStart(arrayDescriptor, extensions, callId)
    return {}
end

return functionCallAndLog("visitArrayPropertyStart", visitArrayPropertyStart, 1)
