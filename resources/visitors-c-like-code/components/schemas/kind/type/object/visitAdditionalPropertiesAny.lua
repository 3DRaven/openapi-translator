--- This visitor handles the processing of free-form `additionalProperties` within an object schema.
--- It deals exclusively with `additionalProperties` that do not have their own schema (free-form).
--- Returns a code for creating storage for additionalProperties (Map as example)
--- @param flag boolean # free form of additionalProperties has this value
--- @param minProperties integer? # minimal number of properties in additionalProperties collection
--- @param maxProperties integer? # maximal number of properties in additionalProperties collection
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitAdditionalPropertiesAny(flag, minProperties, maxProperties, extensions, callId)
    --- @type ModelBase
    local currentModel = GLOBAL_CONTEXT.models:element()
    if currentModel == nil then
        error("additionalProperties with type any in unknown position")
    else
        return CODEGEN.addAdditionalProperty(currentModel, CODE.getAnyType(), extensions)
    end
end

return functionCallAndLog("visitAdditionalPropertiesAny", visitAdditionalPropertiesAny)
