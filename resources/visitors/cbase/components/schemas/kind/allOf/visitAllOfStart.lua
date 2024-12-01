--- This visitor is invoked before processing allOf schemas
--- @param schemas ReferenceOr<Schema>[] #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitAllOfStart(schemas, extensions, callId)
    local parentModel = GLOBAL_CONTEXT.models:peek()
    if parentModel ~= nil and parentModel:instanceOf(AnySchemaModel) then
        -- in unknown schema struct any known combination of properties will be processed as separated
        -- sets of properties with different names
        GLOBAL_CONTEXT.names:push("AnySchemaAllOf")
    end
    GLOBAL_CONTEXT.models:push(AllOfModel.new(concatStackCapitalized(GLOBAL_CONTEXT.names)))
    return {}
end

return functionCallAndLog("visitAllOfStart", visitAllOfStart, 1)
