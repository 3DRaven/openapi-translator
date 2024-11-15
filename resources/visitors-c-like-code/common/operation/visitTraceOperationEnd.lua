--- This visitor is invoked after TRACE operation
--- @param operation Operation
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitTraceOperationEnd(operation, extensions, callId)
    return {}
end

return functionCallAndLog("visitTraceOperationEnd", visitTraceOperationEnd)
