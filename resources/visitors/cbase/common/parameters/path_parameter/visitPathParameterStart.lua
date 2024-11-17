--- This visitor is invoked before processing path type of parameter
--- @param parameterName string|null #
--- @param parameter PathParameter # Represents the headers parameter, which is a map from strings to references or items.
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitPathParameterStart(parameterName, parameter, extensions, callId)
    return {}
end

return functionCallAndLog("visitPathParameterStart", visitPathParameterStart)