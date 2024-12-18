--- This visitor is invoked after processing allOf schemas
--- @param schemas ReferenceOr<Schema>[] #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitAllOfEnd(schemas, extensions, callId)
    ---@type AllOfModel
    local currentModel = GLOBAL_CONTEXT.models:pop() or error("Model for allOf not found")
    if not currentModel:instanceOf(AllOfModel) then
        error("Not allOf model found")
    end

    local parentModel = GLOBAL_CONTEXT.models:peek()
    if parentModel ~= nil and parentModel:instanceOf(AnySchemaModel) then
        -- see visitAllOfStart
        GLOBAL_CONTEXT.names:pop()
    end

    local codeVariant = CODE.getVariant(extensions[Extensions.VARIANT])
    return concatTables(
        currentModel.includes.items,
        { WriteOperation.new_append(codeVariant:getClassHeader(currentModel.name), currentModel.name) },
        currentModel:collectAllPropertiesCode(),
        currentModel.methods.items,
        { WriteOperation.new_append(codeVariant:getClassFooter(),
            currentModel.name) })
end

return functionCallAndLog("visitAllOfEnd", visitAllOfEnd, -1)
