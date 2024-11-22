--- This visitor is invoked before processing header type of parameter
--- @param parameterName string|null #
--- @param parameter HeaderParameter # Represents the headers parameter, which is a map from strings to references or items.
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitHeaderParameterStart(parameterName, parameter, extensions, callId)
    return {}
end

return functionCallAndLog("visitHeaderParameterStart", visitHeaderParameterStart)
