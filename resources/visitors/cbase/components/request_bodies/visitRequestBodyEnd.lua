--- This visitor is invoked after processing request body
--- @param requestBodyName string|null #
--- @param requestBody RequestBody #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitRequestBodyEnd(requestBodyName, requestBody, extensions, callId)
    return {}
end

return functionCallAndLog("visitRequestBodyEnd", visitRequestBodyEnd, -1)
