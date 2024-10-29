--- This visitor is invoked before processing allOf element
--- @param schemas ReferenceOr<Schema>[] #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitAllOfEnd(schemas, extensions, callsStack)
    ---@type AllOfModel
    local currentModel = global_context.models:pop()

    if currentModel == nil then
        error("Model for allOf not found")
    else
        return concatTables(
            currentModel.includes.items,
            { WriteOperation.new_append(string.format("public class %s {\n\n", currentModel.name), currentModel.name) },
            currentModel:collectAllPropertiesCode(),
            currentModel.methods.items,
            { WriteOperation.new_append("\n}\n", currentModel.name) })
    end
end

return functionCallAndLog("visitAllOfEnd", visitAllOfEnd)
