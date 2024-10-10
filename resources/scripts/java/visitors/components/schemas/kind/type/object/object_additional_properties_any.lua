--- This visitor handles the processing of free-form `additionalProperties` within an object schema.
--- It deals exclusively with `additionalProperties` that do not have their own schema (free-form).
--- Returns a code for creating storage for additionalProperties (Map as example)
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param flag boolean # free form of additionalProperties has this value
--- @param minProperties integer? # minimal number of properties in additionalProperties collection
--- @param maxProperties integer? # maximal number of properties in additionalProperties collection
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitAdditionalPropertiesAny(namesStack, flag, minProperties, maxProperties, extensions)
    local parentModelName = getParentModelName(namesStack)
    if parentModelName == nil then
        error("additionalProperties with type any without parent")
    else
        -- Adding the import at the beginning of the parent model file
        global_context:addIncludes("visitAdditionalPropertiesAny", parentModelName,
            { WriteOperation.new_prepend("import java.util.concurrent.ConcurrentHashMap;\n\n", parentModelName) })

        local code = string.format("    private ConcurrentHashMap<String,Object> %s = new ConcurrentHashMap<>();\n",
            getCurrentPropertyNameMandatory(namesStack));

        global_context:addProperties("visitAdditionalPropertiesAny", parentModelName,
            { WriteOperation.new_append(code, parentModelName) })
    end
    return {}
end

return functionCallAndLog("visitAdditionalPropertiesAny", visitAdditionalPropertiesAny)
