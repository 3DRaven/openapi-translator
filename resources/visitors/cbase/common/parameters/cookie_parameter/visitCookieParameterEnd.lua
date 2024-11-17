--- This visitor is invoked after processing cookie type of parameter
--- @param parameterName string|null #
--- @param parameter CookieParameter # Represents the headers parameter, which is a map from strings to references or items.
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitCookieParameterEnd(parameterName, parameter, extensions, callId)
    return {}
end

return functionCallAndLog("visitCookieParameterEnd", visitCookieParameterEnd)