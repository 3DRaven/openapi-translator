--- This visitor is invoked before processing the found schema of type object.
--- Returns a code of the start of an object based on whether it's required.
--- @param objectDescriptor ObjectType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitObjectStart(objectDescriptor, extensions, callId)
    --- When we begin processing an schema, the model might already exist because, each time a
    --- reference is encountered in the specification, the translator starts constructing the model
    --- from scratch. However, the actual text that the reference points to is read only once and cached.
    printBreak()
    printTable(GLOBAL_CONTEXT.names)
    local model = ObjectModel.new(concatStackCapitalized(GLOBAL_CONTEXT.names))
    model.required = objectDescriptor.required
    GLOBAL_CONTEXT.models:push(model)
    return { WriteOperation.new_remove(model.name) }
end

local function beforeDecorator()
end

return functionCallAndLog("visitObjectStart", visitObjectStart, beforeDecorator)
