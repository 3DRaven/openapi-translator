--- This visitor is invoked before starting the processing of the found `additionalProperties` within a schema
--- of type object. This visitor applies to the processing of `additionalProperties` that have their own schema.
--- There is a separate visitor for free-form `additionalProperties`
--- Returns a code for creating model for additionalProperties start part
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param minProperties integer? # minimal number of properties in additionalProperties collection
--- @param maxProperties integer? # maximal number of properties in additionalProperties collection
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitObjectAdditionalPropertiesStart(namesStack, minProperties, maxProperties, extensions)
    return {}
end

local function beforeDecorator()
    global_context:addParentType("visitObjectAdditionalPropertiesStart", ParentType.ADDITIONAL)
end

return functionCallAndLog("visitObjectAdditionalPropertiesStart", visitObjectAdditionalPropertiesStart, beforeDecorator)
