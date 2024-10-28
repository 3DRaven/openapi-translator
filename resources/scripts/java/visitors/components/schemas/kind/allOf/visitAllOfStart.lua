--- This visitor is invoked before processing allOf schemas
--- @param schemas ReferenceOr<Schema>[] #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitAllOfStart(schemas, extensions, callsStack)
    --- When we begin processing an object, the model might already exist because, each time a
    --- reference is encountered in the specification, the translator starts constructing the model
    --- from scratch. However, the actual text that the reference points to is read only once and cached.
    local currentModelName = getCurrentModelNameMandatory(namesStack)
    global_context:dropModel("visitAllOfStart", currentModelName)
    return { WriteOperation.new_remove(currentModelName) }
end

return functionCallAndLog("visitAllOfStart", visitAllOfStart)
