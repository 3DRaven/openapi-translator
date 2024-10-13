--- This visitor is invoked for every server variable
--- @param variableName string # all descrivbed servers in spec
--- @param enumeration string[] # An enumeration of string values for limited set substitution options.
--- @param default string # REQUIRED. The default value to use for substitution if an alternate is not supplied.
--- @param description string|nil # An optional description for the server variable.
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecServerVariable(variableName, enumeration, default, description, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSpecServerVariable", visitSpecServerVariable)
