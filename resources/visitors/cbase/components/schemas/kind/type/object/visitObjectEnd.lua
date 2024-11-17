--- @param model ModelBase
--- @return WriteOperation[] # final code
local function getCollectedCode(model)
    return concatTables(
        model.includes.items,
        { WriteOperation.new_append(CODE.getClassHeader(model.name),
            model.name) },
        model:collectAllPropertiesCode(),
        model.methods.items,
        { WriteOperation.new_append(CODE.getClassFooter(), model.name) })
end

--- This visitor is invoked after all the content inside a schema of type object has been processed.
--- Returns a code of the end of an object based on whether it's required.
--- @param objectDescriptor ObjectType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitObjectEnd(objectDescriptor, extensions, callId)
    --- The endpoint for this visitor is either that the code will be saved to disk, or that
    --- the model will be transferred to the parent, so we can immediately delete the current model.
    --- @type ModelBase
    local currentModel = GLOBAL_CONTEXT.models:pop()
    --- @type ModelBase?
    local parentModel = GLOBAL_CONTEXT.models:peek()

    -- if this object has parent, it must save something to parent about it self
    if parentModel ~= nil then
        -- For parent OBJECT we need to write property to it
        if parentModel:instanceOf(ObjectModel) then
            VISITORS.struct.addGenericPropertyCode(parentModel, currentModel.name, extensions)
        elseif parentModel:instanceOf(TypeTransferModel) then
            -- object must write it self model (currentModel) and must send it type to parent object
            parentModel.name = currentModel.name
        elseif parentModel:instanceOf(AllOfModel) then
            --- If the parent is allOf, we need to place all created properties and other of this object into the parent.
            parentModel:includeModel(currentModel)
            return {}
        elseif parentModel:instanceOf(OneOfModel) then
            -- for "oneOf" parent model we just save created model as is, because all models must be created for select one of it
        end
    end

    return getCollectedCode(currentModel)
end

return functionCallAndLog("visitObjectEnd", visitObjectEnd)
