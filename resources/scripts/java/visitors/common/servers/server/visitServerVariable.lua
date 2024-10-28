--- This visitor is invoked for every server variable
--- @param serverUrl string # REQUIRED. A URL to the target host. Supports Server Variables and MAY be relative.
--- @param variableName string # all descrivbed servers in spec
--- @param variable ServerVariable # An enumeration of string values for limited set substitution options.
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitServerVariable(serverUrl, variableName, variable, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitServerVariable", visitServerVariable)
