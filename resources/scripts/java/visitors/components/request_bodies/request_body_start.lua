--- This visitor is invoked before processing request body
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param requestBody RequestBody #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitRequestBodyStart(namesStack, requestBody, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitRequestBodyStart", visitRequestBodyStart)
