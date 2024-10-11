--- This visitor is invoked after processing all response headers
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param headers table<string, Header> # Represents the headers parameter, which is a map from strings to references or items.
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitResponseHeadersEnd(namesStack, headers, extensions)
    return {}
end

return functionCallAndLog("visitResponseHeadersEnd", visitResponseHeadersEnd)