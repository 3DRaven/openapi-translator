--- This visitor is invoked after processing response header if reference
--- @param headerName string|null #
--- @param headerReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitHeaderReferenceEnd(headerName, headerReference, extensions, callId)
    return {}
end

return functionCallAndLog("visitHeaderReferenceEnd", visitHeaderReferenceEnd, -1)
