--- This visitor is invoked before processing async callback
--- @param callbackName string|null
--- @param callback table<string, PathItem> #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitAsyncCallbackEnd(callbackName, callback, extensions, callsStack)
    
    return {}
end

return functionCallAndLog("visitAsyncCallbackEnd", visitAsyncCallbackEnd)
