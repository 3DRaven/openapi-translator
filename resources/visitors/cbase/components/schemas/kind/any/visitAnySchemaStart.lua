--- This visitor is invoked to process a found schema without a defined structure
--- such as `additionalProperties: {}`.
--- Returns a model name with unknown structure (it is {} in OpenAPI)
--- @param anySchemaDescriptor AnySchema # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitAnySchemaStart(anySchemaDescriptor, extensions, callId)
    --- @type ModelBase?
    local currentModel = GLOBAL_CONTEXT.models:peek()
    if currentModel ~= nil then
        if currentModel:instanceOf(TypeTransferModel) then
            currentModel.name = CODE.getAnyType()
        else
            error("Unknown model for Any schema")
        end
    else
        print("Root model for Any schema")
    end
    return {}
end

return functionCallAndLog("visitAnySchemaStart", visitAnySchemaStart)
