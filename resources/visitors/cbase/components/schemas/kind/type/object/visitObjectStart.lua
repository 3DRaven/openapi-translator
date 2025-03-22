--- This visitor is invoked before processing the found schema of type object.
--- Returns a code of the start of an object based on whether it's required.
--- @param objectDescriptor ObjectType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitObjectStart(objectDescriptor, extensions, callId)
    -- as example in allOf objects does not generate real models with names, it is just lists of properties
    -- so, we do not have real model names for them
    local modelName = concatStackCapitalized(GLOBAL_CONTEXT.names) or "unknown-object"
    local model = ObjectModel.new(modelName,objectDescriptor.required, objectDescriptor,extensions)
    GLOBAL_CONTEXT.values:push(model)
    return {}
end

return functionCallAndLog("visitObjectStart", visitObjectStart, 1)