--- This visitor is invoked before processing oneOf schemas
--- @param schemas ReferenceOr<Schema>[] #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitOneOfStart(schemas, extensions, callsStack)
    GLOBAL_CONTEXT.models:push(OneOfModel.new("stub-oneOf-model-name"))
    return {}
end

return functionCallAndLog("visitOneOfStart", visitOneOfStart)
