--- This visitor is invoked before processing all response header examples
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param examples table<string, Example> # Represents the headers parameter, which is a map from strings to references or items.
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitResponseHeaderExamplesStart(namesStack, examples, extensions)
    return {}
end

return functionCallAndLog("visitResponseHeaderExamplesStart", visitResponseHeaderExamplesStart)
