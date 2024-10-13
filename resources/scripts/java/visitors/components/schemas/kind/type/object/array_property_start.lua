--- Represents an array type used in schema definitions.
--- @class ArrayDescriptor
--- @field min_items integer | nil   # number Optional. Specifies the minimum number of items in the array.
--- @field max_items integer | nil   # number Optional. Specifies the maximum number of items in the array.
--- @field unique_items boolean     # Indicates whether all items in the array must be unique.

--- This visitor is invoked to start processing a schema containing an array, regardless of whether the schema
--- is nested within an object or is a top-level schema. The invocation occurs before processing the schemas
--- contained within the array.
--- Returns a code of start for array property of object
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param arrayDescriptor ArrayDescriptor # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitArrayPropertyStart(namesStack, arrayDescriptor, extensions, callsStack)
    local parentModelName = getParentModelName(namesStack)
    -- if it is root object as array we must clean old variant of it model
    if parentModelName == nil then
        return { WriteOperation.new_remove(getCurrentModelNameMandatory(namesStack)) }
    else
        return {}
    end
end

local function beforeDecorator()
    global_context:addParentType("visitArrayPropertyStart", ParentType.ARRAY)
end

return functionCallAndLog("visitArrayPropertyStart", visitArrayPropertyStart, beforeDecorator)
