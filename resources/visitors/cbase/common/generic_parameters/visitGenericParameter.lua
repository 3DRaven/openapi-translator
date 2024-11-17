--- This visitor is invoked for processing generic parameter
--- @param parameterName string
--- @param parameter table
--- @param extensions table             # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[]            # Returns the output code and  file name for writing code
local function visitGenericParameter(parameterName, parameter, extensions, callId)
    return {}
end

return functionCallAndLog("visitGenericParameter", visitGenericParameter)
