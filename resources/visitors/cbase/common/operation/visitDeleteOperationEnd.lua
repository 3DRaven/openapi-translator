--- This visitor is invoked after DELETE operation
--- @param operation Operation
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitDeleteOperationEnd(operation, extensions, callId)
    return {}
end

return functionCallAndLog("visitDeleteOperationEnd", visitDeleteOperationEnd)
