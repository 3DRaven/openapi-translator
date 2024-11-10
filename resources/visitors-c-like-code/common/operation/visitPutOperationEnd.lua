--- This visitor is invoked after PUT operation
--- @param operation Operation
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitPutOperationEnd(operation, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitPutOperationEnd", visitPutOperationEnd)
