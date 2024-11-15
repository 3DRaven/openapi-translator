--- This visitor is invoked when a property of type integer is found.
--- Returns a code for creating integer value property of object
--- @param integerDescriptor IntegerType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitIntegerProperty(integerDescriptor, extensions, callId)
    return CODEGEN.addGenericPropertyCode(GLOBAL_CONTEXT.models:peek(), CODE.getIntegerType(), extensions)
end

return functionCallAndLog("visitIntegerProperty", visitIntegerProperty)
