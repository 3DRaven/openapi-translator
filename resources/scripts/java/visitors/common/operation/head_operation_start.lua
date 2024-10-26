--- This visitor is invoked before HEAD operation
--- @param server Server # REQUIRED. A URL to the target host. Supports Server Variables and MAY be relative.
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitHeadOperationStart(server, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitHeadOperationStart", visitHeadOperationStart)
