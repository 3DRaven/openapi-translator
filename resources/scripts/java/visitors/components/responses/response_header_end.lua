--- This visitor is invoked after processing responses header
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitResponseHeaderEnd(extensions)
    return {}
end

return functionCallAndLog("visitResponseHeaderEnd", visitResponseHeaderEnd)
