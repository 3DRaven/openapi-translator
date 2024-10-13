--- @class ObjectDescriptor
--- @field properties table<string, table> # Table of properties
--- @field required string[]|nil # List of required properties
--- @field additional_properties table|boolean|nil # Additional properties if present.
--- @field min_properties number|nil # Minimum number of properties if specified.
--- @field max_properties number|nil # Maximum number of properties if specified.

--- This visitor is invoked before processing the found schema of type object.
--- Returns a code of the start of an object based on whether it's required.
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param objectDescriptor ObjectDescriptor # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitObjectStart(namesStack, objectDescriptor, extensions, callsStack)
    --- When we begin processing an object, the model might already exist because, each time a
    --- reference is encountered in the specification, the translator starts constructing the model
    --- from scratch. However, the actual text that the reference points to is read only once and cached.
    local currentModelName = getCurrentModelNameMandatory(namesStack)
    global_context:dropModel("visitObjectStart", currentModelName)
    global_context:setRequiredProperties("visitObjectStart", currentModelName, objectDescriptor.required)
    return { WriteOperation.new_remove(currentModelName) }
end

local function beforeDecorator(namesStack)
    global_context:addParentType("visitObjectStart", ParentType.OBJECT)
end

return functionCallAndLog("visitObjectStart", visitObjectStart, beforeDecorator)
