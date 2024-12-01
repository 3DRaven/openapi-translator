--- This visitor is invoked after processing request body if reference
--- @param requestBodyName string|null #
--- @param requestBodyReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitRequestBodyReferenceEnd(requestBodyName, requestBodyReference, extensions, callId)
    return {}
end

return functionCallAndLog("visitRequestBodyReferenceEnd", visitRequestBodyReferenceEnd, -1)
