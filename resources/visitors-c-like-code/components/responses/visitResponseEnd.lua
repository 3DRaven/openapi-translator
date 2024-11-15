--- This visitor is invoked after processing response
--- @param responseName string|null #
--- @param response Response # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitResponseEnd(responseName, response, extensions, callId)
    return {}
end

return functionCallAndLog("visitResponseEnd", visitResponseEnd)
