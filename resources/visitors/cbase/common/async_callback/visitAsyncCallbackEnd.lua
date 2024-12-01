--- This visitor is invoked before processing async callback
--- @param callbackName string|null
--- @param callback table<string, PathItem> #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitAsyncCallbackEnd(callbackName, callback, extensions, callId)
    
    return {}
end

return functionCallAndLog("visitAsyncCallbackEnd", visitAsyncCallbackEnd, -1)
