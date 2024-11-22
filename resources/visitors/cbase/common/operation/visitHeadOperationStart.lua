--- This visitor is invoked before HEAD operation
--- @param operation Operation
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitHeadOperationStart(operation, extensions, callId)
    return {}
end

return functionCallAndLog("visitHeadOperationStart", visitHeadOperationStart)
