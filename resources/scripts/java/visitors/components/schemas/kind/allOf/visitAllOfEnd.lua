--- This visitor is invoked before processing allOf element
--- @param schemas ReferenceOr<Schema>[] #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitAllOfEnd(schemas, extensions, callsStack)
    local currentModelName = getCurrentModelNameMandatory(namesStack)

    -- this "object" must to save self model
    local model = global_context:getModelByName("visitAllOfEnd", currentModelName)
    if model == nil then
        error("Model for allOf not found")
    else
        return concatTables(
            model.includes,
            { WriteOperation.new_append(string.format("public class %s {\n\n", currentModelName), currentModelName) },
            model.properties,
            model.methods,
            { WriteOperation.new_append("\n}\n", currentModelName) })
    end
end

return functionCallAndLog("visitAllOfEnd", visitAllOfEnd)
