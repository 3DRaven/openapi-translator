--- This visitor is invoked before processing response
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitResponseStart(extensions)
    return {}
end

return functionCallAndLog("visitResponseStart", visitResponseStart)
