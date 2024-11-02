--- This visitor is invoked to process a found schema without a defined structure,
--- such as `additionalProperties: {}`.
--- Returns a model name with unknown structure (it is {} in OpenAPI)
--- @param anySchemaDescriptor AnySchema # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitAnySchema(anySchemaDescriptor, extensions, callsStack)
    --- @type ModelBase
    local currentModel = GLOBAL_CONTEXT.models:element()
    if currentModel:instanceOf(TypeTransferModel) then
        currentModel.name = CODE.getAnyType()
    else
        error("Unknown model for Any schema")
    end
    return {}
end

return functionCallAndLog("visitAnySchema", visitAnySchema)
