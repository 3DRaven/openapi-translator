--- This visitor is invoked after processing schemas
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSchemasEnd(extensions)
    return {}
end

return functionCallAndLog("visitSchemasEnd", visitSchemasEnd)
