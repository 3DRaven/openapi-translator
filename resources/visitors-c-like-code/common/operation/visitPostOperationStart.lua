--- This visitor is invoked before POST operation
--- @param operation Operation
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitPostOperationStart(operation, extensions, callId)
    return {}
end

return functionCallAndLog("visitPostOperationStart", visitPostOperationStart)
