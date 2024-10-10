--- This visitor is invoked at example of schema
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param example table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitPathStart(namesStack, example, extensions)
    return {}
end

return functionCallAndLog("visitPathStart", visitPathStart)
