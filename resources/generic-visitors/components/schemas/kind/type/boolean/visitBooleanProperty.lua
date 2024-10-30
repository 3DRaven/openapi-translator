--- This visitor is invoked when a property of type boolean is found.
--- Returns a code for creating storage for additionalProperties (Map as example)
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param booleanDescriptor BooleanType # boolean property descriptor
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitBooleanProperty(booleanDescriptor, extensions, callsStack)
    return CODEGEN.addGenericPropertyCode(global_context.models:element(), "Boolean", extensions)
end

return functionCallAndLog("visitBooleanProperty", visitBooleanProperty)
