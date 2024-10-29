--- This visitor is invoked to start processing a schema containing an array, regardless of whether the schema
--- is nested within an object or is a top-level schema. The invocation occurs before processing the schemas
--- contained within the array.
--- Returns a code of start for array property of object
--- @param arrayDescriptor ArrayType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitArrayPropertyStart(arrayDescriptor, extensions, callsStack)
    -- This is a temporary model for collecting information about the schemas inside the array
    global_context.models:push(TypeTransferModel.new("unknown-items"))
    return {}
end

local function beforeDecorator()
end

return functionCallAndLog("visitArrayPropertyStart", visitArrayPropertyStart, beforeDecorator)
