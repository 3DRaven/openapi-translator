--- This visitor is invoked before processing allOf schemas
--- @param schemas ReferenceOr<Schema>[] #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitAllOfStart(schemas, extensions, callsStack)
    GLOBAL_CONTEXT.models:push(AllOfModel.new(GLOBAL_CONTEXT.names:element()))
    return {}
end

return functionCallAndLog("visitAllOfStart", visitAllOfStart)
