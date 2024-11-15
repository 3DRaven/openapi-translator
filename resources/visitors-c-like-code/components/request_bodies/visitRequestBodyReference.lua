--- This visitor is invoked before processing request body if reference
--- @param requestBodyName string|null #
--- @param requestBodyReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitRequestBodyReference(requestBodyName, requestBodyReference, extensions, callId)
    return {}
end

return functionCallAndLog("visitRequestBodyReference", visitRequestBodyReference)
