--- This visitor is invoked after processing async callback
--- @param callbackName string|null
--- @param callbackReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitAsyncCallbackReferenceEnd(callbackName, callbackReference, extensions, callId)
    return {}
end

return functionCallAndLog("visitAsyncCallbackReferenceEnd", visitAsyncCallbackReferenceEnd)
