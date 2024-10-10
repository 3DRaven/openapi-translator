--- This visitor is invoked after processing response
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitResponseEnd(extensions)
    return {}
end

return functionCallAndLog("visitResponseEnd", visitResponseEnd)
