--- This visitor is invoked after POST operation
--- @param operation Operation
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitPostOperationEnd(operation, extensions, callId)
    return {}
end

return functionCallAndLog("visitPostOperationEnd", visitPostOperationEnd, -1)
