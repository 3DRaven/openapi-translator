--- This visitor is invoked before processing parameter if reference
--- @param parameterName string|null #
--- @param parameterReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitParameterReferenceStart(parameterName, parameterReference, extensions, callId)
    return {}
end

return functionCallAndLog("visitParameterReferenceStart", visitParameterReferenceStart)
