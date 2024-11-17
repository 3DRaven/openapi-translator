--- This visitor is invoked before processing any response
--- @param responses table<string,ReferenceOr<Response>>
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitResponsesStart(responses, extensions, callId)
    return {}
end

return functionCallAndLog("visitResponsesStart", visitResponsesStart)
