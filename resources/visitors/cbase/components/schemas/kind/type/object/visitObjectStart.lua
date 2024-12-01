--- This visitor is invoked before processing the found schema of type object.
--- Returns a code of the start of an object based on whether it's required.
--- @param objectDescriptor ObjectType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitObjectStart(objectDescriptor, extensions, callId)
    --- @type ModelBase?
    local parentModel = GLOBAL_CONTEXT.models:peek()
    -- as example in allOf objects does not generate real models with names, it is just lists of properties
    -- so, we do not have real model names for them
    local modelName = concatStackCapitalized(GLOBAL_CONTEXT.names) or "unknown-object"
    local model = ObjectModel.new(modelName)
    model.required = objectDescriptor.required
    GLOBAL_CONTEXT.models:push(model)
    if parentModel ~= nil and parentModel:instanceOf(AllOfModel) then
        return {}
    else
        --- When we begin processing an schema, the model might already exist because, each time a
        --- reference is encountered in the specification, the translator starts constructing the model
        --- from scratch. However, the actual text that the reference points to is read only once and cached.
        return { WriteOperation.new_remove(modelName) }
    end
end

return functionCallAndLog("visitObjectStart", visitObjectStart, 1)
