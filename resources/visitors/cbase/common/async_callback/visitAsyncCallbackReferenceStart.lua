--- This visitor is invoked before processing async callback
--- @param callbackName string|null
--- @param callbackReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitAsyncCallbackReferenceStart(callbackName, callbackReference, extensions, callId)
    return {}
end

return functionCallAndLog("visitAsyncCallbackReferenceStart", visitAsyncCallbackReferenceStart)
