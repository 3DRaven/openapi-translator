--- This visitor is invoked at default value of schema
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param default table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitDefault(namesStack, default, extensions)
    return {}
end

return functionCallAndLog("visitDefault", visitDefault)
