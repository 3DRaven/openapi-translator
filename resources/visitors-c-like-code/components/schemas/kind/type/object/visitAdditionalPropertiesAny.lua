--- This visitor handles the processing of free-form `additionalProperties` within an object schema.
--- It deals exclusively with `additionalProperties` that do not have their own schema (free-form).
--- Returns a code for creating storage for additionalProperties (Map as example)
--- @param flag boolean # free form of additionalProperties has this value
--- @param minProperties integer? # minimal number of properties in additionalProperties collection
--- @param maxProperties integer? # maximal number of properties in additionalProperties collection
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitAdditionalPropertiesAny(flag, minProperties, maxProperties, extensions, callsStack)
    --- @type ModelBase
    local currentModel = GLOBAL_CONTEXT.models:element()
    if currentModel == nil then
        error("additionalProperties with type any in unknown position")
    else
        return CODEGEN.addAdditionalProperty(currentModel, CODE.getAnyType(), extensions)
    end
end

return functionCallAndLog("visitAdditionalPropertiesAny", visitAdditionalPropertiesAny)
