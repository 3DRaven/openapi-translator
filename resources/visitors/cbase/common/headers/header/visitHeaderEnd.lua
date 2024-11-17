--- This visitor is invoked after processing response header
--- @param headerName string|null #
--- @param header Header #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitHeaderEnd(headerName, header, extensions, callId)
    return {}
end

return functionCallAndLog("visitHeaderEnd", visitHeaderEnd)
