--- This visitor is invoked before processing response header if reference
--- @param headerName string|null #
--- @param headerReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitHeaderReferenceStart(headerName, headerReference, extensions, callId)
    return {}
end

return functionCallAndLog("visitHeaderReferenceStart", visitHeaderReferenceStart)
