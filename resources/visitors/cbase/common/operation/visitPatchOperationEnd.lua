--- This visitor is invoked after PATCH operation
--- @param operation Operation
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitPatchOperationEnd(operation, extensions, callId)
    return {}
end

return functionCallAndLog("visitPatchOperationEnd", visitPatchOperationEnd)
