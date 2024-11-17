--- This visitor is invoked to start processing a schema containing an array, regardless of whether the schema
--- is nested within an object or is a top-level schema. The invocation occurs before processing the schemas
--- contained within the array.
--- Returns a code of start for array property of object
--- @param arrayDescriptor ArrayType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitArrayPropertyStart(arrayDescriptor, extensions, callId)
    --- @type ModelBase?
    -- This is a temporary model for collecting information about the schemas inside the array
    GLOBAL_CONTEXT.models:push(TypeTransferModel.new("unknown-items"))

    return {}
end

local function beforeDecorator()
end

return functionCallAndLog("visitArrayPropertyStart", visitArrayPropertyStart, beforeDecorator)