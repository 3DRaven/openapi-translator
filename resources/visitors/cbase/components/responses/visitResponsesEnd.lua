--- This visitor is invoked after processing responses
--- @param responses table<string,ReferenceOr<Response>>
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitResponsesEnd(responses, extensions, callId)
    return {}
end

return functionCallAndLog("visitResponsesEnd", visitResponsesEnd)
