--- This visitor is invoked before processing allOf element
--- @param schemas ReferenceOr<Schema>[] #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitAllOfEnd(schemas, extensions, callId)
    ---@type AllOfModel
    local currentModel = GLOBAL_CONTEXT.models:pop()

    if currentModel == nil then
        error("Model for allOf not found")
    else
        return concatTables(
            currentModel.includes.items,
            { WriteOperation.new_append(CODE.getClassHeader(currentModel.name), currentModel.name) },
            currentModel:collectAllPropertiesCode(),
            currentModel.methods.items,
            { WriteOperation.new_append(CODE.getClassFooter(), currentModel.name) })
    end
end

return functionCallAndLog("visitAllOfEnd", visitAllOfEnd)
