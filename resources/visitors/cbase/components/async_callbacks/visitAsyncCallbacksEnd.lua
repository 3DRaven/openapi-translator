--- This visitor is invoked after processing operation callbacks
--- @param operationCallbacks OperationCallbacks # Represents the headers parameter, which is a map from strings to references or items.
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitAsyncCallbacksEnd(namesStack, operationCallbacks, extensions, callId)
    return {}
end

return functionCallAndLog("visitAsyncCallbacksEnd", visitAsyncCallbacksEnd, -1)
