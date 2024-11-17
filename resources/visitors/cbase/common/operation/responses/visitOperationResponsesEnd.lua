--- This visitor is invoked after operation responses
--- @param responses Responses #
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitOperationResponsesEnd(responses, extensions, callId)
    return {}
end

return functionCallAndLog("visitOperationResponsesEnd", visitOperationResponsesEnd)
