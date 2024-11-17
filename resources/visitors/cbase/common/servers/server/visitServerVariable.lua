--- This visitor is invoked for every server variable
--- @param serverUrl string # REQUIRED. A URL to the target host. Supports Server Variables and MAY be relative.
--- @param variableName string # all descrivbed servers in spec
--- @param variable ServerVariable # An enumeration of string values for limited set substitution options.
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitServerVariable(serverUrl, variableName, variable, extensions, callId)
    return {}
end

return functionCallAndLog("visitServerVariable", visitServerVariable)
