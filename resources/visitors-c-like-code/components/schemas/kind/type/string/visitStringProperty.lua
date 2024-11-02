--- This visitor is invoked when a property of type string is found.
--- Returns a string based on the provided string descriptor.
--- @param stringDescriptor StringType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitStringProperty(stringDescriptor, extensions, callsStack)
    return CODEGEN.addGenericPropertyCode(GLOBAL_CONTEXT.models:element(), CODE.getStringType(), extensions)
end

return functionCallAndLog("visitStringProperty", visitStringProperty)
