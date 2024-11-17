--- This visitor handles the processing `additionalProperties` within an object schema.
--- Called before processing schema of `additionalProperties`
--- @param schema ReferenceOr<Schema> #
--- @param minProperties integer? # minimal number of properties in additionalProperties collection
--- @param maxProperties integer? # maximal number of properties in additionalProperties collection
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitAdditionalPropertiesStart(schema, minProperties, maxProperties, extensions, callId)
    -- This is a temporary model for collecting information about the schemas inside the additionalProperties
    GLOBAL_CONTEXT.models:push(TypeTransferModel.new("unknown-additional-properties"))
    return {}
end

return functionCallAndLog("visitAdditionalPropertiesStart", visitAdditionalPropertiesStart)