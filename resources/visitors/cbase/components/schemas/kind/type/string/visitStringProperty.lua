--- This visitor is invoked when a property of type string is found.
--- Returns a string based on the provided string descriptor.
--- @param stringDescriptor StringType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitStringProperty(stringDescriptor, extensions, callId)
    local propertyName = GLOBAL_CONTEXT.names:pop()
    --- @type ObjectModel
    local currentModel = GLOBAL_CONTEXT.values:element()
    assert(currentModel:instanceOf(ObjectModel), "Found not a ObjectModel")
    GLOBAL_CONTEXT.values:push(STRUCT.createProperty(currentModel, propertyName, stringDescriptor, extensions))
    return {}
end

return functionCallAndLog("visitStringProperty", visitStringProperty)
