--- This visitor is invoked before processing any response
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitResponsesStart(extensions)
    return {}
end

return functionCallAndLog("visitResponsesStart", visitResponsesStart)
