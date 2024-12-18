--- This visitor is invoked when a property of type string is found.
--- Returns a string based on the provided string descriptor.
--- @param stringDescriptor StringType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitStringProperty(stringDescriptor, extensions, callId)
    local codeVariant = CODE.getVariant(extensions[Extensions.VARIANT])
    return STRUCT.addGenericPropertyCode(GLOBAL_CONTEXT.models:peek(), codeVariant:getStringType(), extensions)
end

return functionCallAndLog("visitStringProperty", visitStringProperty)
