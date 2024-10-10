--- This visitor is invoked after processing any kind of schema
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param schemaDescriptor SchemaDescriptor # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSchemaEnd(namesStack, schemaDescriptor, extensions)
    return {}
end

return functionCallAndLog("visitSchemaEnd", visitSchemaEnd)
