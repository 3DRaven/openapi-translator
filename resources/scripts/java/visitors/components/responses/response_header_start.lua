--- This visitor is invoked before processing response header
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitResponseHeaderStart(extensions)
    return {}
end

return functionCallAndLog("visitResponseHeaderStart", visitResponseHeaderStart)
