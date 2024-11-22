--- This visitor is invoked after processing parameter if reference
--- @param parameterName string|null #
--- @param parameterReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitParameterReferenceEnd(parameterName, parameterReference, extensions, callId)
    return {}
end

return functionCallAndLog("visitParameterReferenceEnd", visitParameterReferenceEnd)
