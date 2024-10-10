--- This visitor is invoked before processing any schemas
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSchemasStart(extensions)
    return {}
end

return functionCallAndLog("visitSchemasStart", visitSchemasStart)
