--- This visitor is invoked before processing response
--- @param responseName string|null #
--- @param response Response # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitResponseStart(responseName, response, extensions, callId)
    return {}
end

return functionCallAndLog("visitResponseStart", visitResponseStart, 1)
