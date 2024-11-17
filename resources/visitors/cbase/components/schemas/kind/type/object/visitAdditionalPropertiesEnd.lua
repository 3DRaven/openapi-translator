--- This visitor handles the processing of`additionalProperties` within an object schema.
--- Called after processing schema of `additionalProperties`
--- @param schema ReferenceOr<Schema> #
--- @param minProperties integer? # minimal number of properties in additionalProperties collection
--- @param maxProperties integer? # maximal number of properties in additionalProperties collection
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitAdditionalPropertiesEnd(schema, minProperties, maxProperties, extensions, callId)
    --- @type ModelBase
    local childModel = GLOBAL_CONTEXT.models:pop()
    --- @type ModelBase
    local currentModel = GLOBAL_CONTEXT.models:element()

    if currentModel:instanceOf(ObjectModel) then
        if childModel:instanceOf(TypeTransferModel) then
            return VISITORS.struct.addAdditionalProperty(currentModel, childModel.name, extensions)
        else
            error("Child type for additionalProperties not found")
        end
    else
        error("additionalProperties not in object found")
    end
end

return functionCallAndLog("visitAdditionalPropertiesEnd", visitAdditionalPropertiesEnd)
