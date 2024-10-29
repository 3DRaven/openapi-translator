--- @param model ModelBase
--- @return WriteOperation[] # final code
local function getCollectedCode(model)
    return concatTables(
        model.includes.items,
        { WriteOperation.new_append(string.format("\npublic class %s {\n\n", model.name),
            model.name) },
        model:collectAllPropertiesCode(),
        model.methods.items,
        { WriteOperation.new_append("\n}\n", model.name) })
end

--- This visitor is invoked after all the content inside a schema of type object has been processed.
--- Returns a code of the end of an object based on whether it's required.
--- @param objectDescriptor ObjectType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitObjectEnd(objectDescriptor, extensions, callsStack)
    --- The endpoint for this visitor is either that the code will be saved to disk, or that
    --- the model will be transferred to the parent, so we can immediately delete the current model.
    --- @type ModelBase
    local currentModel = global_context.models:pop()
    ---@type ModelBase?
    local parentModel = global_context.models:peek()

    -- if this object has parent, it must save something to parent about it self
    if parentModel ~= nil then
        -- For parent OBJECT we need to write property to it
        if parentModel:instanceOf(ObjectModel) then
            addGenericPropertyCode(parentModel, currentModel.name, extensions)
        elseif parentModel:instanceOf(AllOfModel) then
            --- If the parent is allOf, we need to place all created properties and other of this object into the parent.
            parentModel:includeModel(currentModel)
            return {}
        end
    end

    return getCollectedCode(currentModel)
end

return functionCallAndLog("visitObjectEnd", visitObjectEnd)
