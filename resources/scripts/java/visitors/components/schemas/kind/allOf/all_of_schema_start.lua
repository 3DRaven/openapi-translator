--- This visitor is invoked before processing the found schema in allOf element
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param required boolean # Indicates if the property value (this object) is required
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitAllOfSchemaStart(namesStack, required, extensions)
    return {}
end

return functionCallAndLog("visitAllOfSchemaStart", visitAllOfSchemaStart)
