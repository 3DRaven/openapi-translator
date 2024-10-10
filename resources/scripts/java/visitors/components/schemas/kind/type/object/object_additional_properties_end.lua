--- This visitor finalizes the processing of `additionalProperties` within an object schema.
--- It applies only to `additionalProperties` with their own schema;
--- there's a separate visitor for free-form `additionalProperties`.
--- This visitor is invoked after the models related to the `additionalProperties` content have been processed.
--- Returns a code for creating model for additionalProperties end part
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param minProperties integer? # minimal number of properties in additionalProperties collection
--- @param maxProperties integer? # maximal number of properties in additionalProperties collection
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitObjectAdditionalPropertiesEnd(namesStack, minProperties, maxProperties, extensions)
    local parentModelName = getParentModelName(namesStack)
    if parentModelName == nil then
        error("additionalProperties without parent")
    else
        -- Adding the import at the beginning of the parent model file
        global_context:addIncludes("visitObjectAdditionalPropertiesEnd", parentModelName,
            { WriteOperation.new_prepend("import java.util.concurrent.ConcurrentHashMap;\n\n", parentModelName) })

        local lastChildrenModelName = global_context:getLastChildrenModelName("visitObjectAdditionalPropertiesEnd")

        local code = string.format("    private ConcurrentHashMap<String,%s> %s = new ConcurrentHashMap<>();\n",
            lastChildrenModelName,
            getCurrentPropertyNameMandatory(namesStack));

        global_context:addProperties("visitObjectAdditionalPropertiesEnd", parentModelName,
            { WriteOperation.new_append(code, parentModelName) })
    end
    return {}
end

local function beforeDecorator()
    -- drop before main code because we need to know parent for this object if it exists, this object not a parent now
    global_context:dropLastParentType("visitObjectAdditionalPropertiesEnd")
end

local function afterDecorator()
    -- last children used and it can be forgotten
    global_context:dropLastChildrenModelName("visitObjectAdditionalPropertiesEnd")
end
return functionCallAndLog("visitObjectAdditionalPropertiesEnd", visitObjectAdditionalPropertiesEnd,
    beforeDecorator, afterDecorator)
