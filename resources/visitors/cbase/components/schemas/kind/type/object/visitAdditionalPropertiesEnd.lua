--- This visitor handles the processing of`additionalProperties` within an object schema.
--- Called after processing schema of `additionalProperties`
--- @param schema ReferenceOr<Schema> #
--- @param minProperties integer? # minimal number of properties in additionalProperties collection
--- @param maxProperties integer? # maximal number of properties in additionalProperties collection
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitAdditionalPropertiesEnd(schema, minProperties, maxProperties, extensions, callId)
    return {}
end

return functionCallAndLog("visitAdditionalPropertiesEnd", visitAdditionalPropertiesEnd, -1)
