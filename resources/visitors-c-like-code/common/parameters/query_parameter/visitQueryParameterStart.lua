--- This visitor is invoked before processing query type of parameter
--- @param parameterName string|null #
--- @param parameter QueryParameter # Represents the headers parameter, which is a map from strings to references or items.
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitQueryParameterStart(parameterName, parameter, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitQueryParameterStart", visitQueryParameterStart)
