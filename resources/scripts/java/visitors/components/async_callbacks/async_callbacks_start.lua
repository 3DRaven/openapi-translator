--- This visitor is invoked before processing operation callbacks
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param operationCallbacks OperationCallbacks # Represents the headers parameter, which is a map from strings to references or items.
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitAsyncCallbacksStart(namesStack, operationCallbacks, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitAsyncCallbacksStart", visitAsyncCallbacksStart)
