--- This visitor handles the processing `additionalProperties` within an object schema.
--- Called before processing schema of `additionalProperties`
--- @param schema ReferenceOr<Schema> #
--- @param minProperties integer? # minimal number of properties in additionalProperties collection
--- @param maxProperties integer? # maximal number of properties in additionalProperties collection
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitAdditionalPropertiesStart(schema, minProperties, maxProperties, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitAdditionalPropertiesStart", visitAdditionalPropertiesStart)
