--- This visitor is invoked after OPTIONS operation
--- @param operation Operation
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitOptionsOperationEnd(operation, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitOptionsOperationEnd", visitOptionsOperationEnd)
