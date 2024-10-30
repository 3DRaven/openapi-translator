--- This visitor is invoked when a property of type number is found.
--- Returns a code for creating property for number value
--- @param numberDescriptor StringType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitNumberProperty(numberDescriptor, extensions, callsStack)
    return CODEGEN.addGenericPropertyCode(global_context.models:element(), "Number", extensions)
end

return functionCallAndLog("visitNumberProperty", visitNumberProperty)
