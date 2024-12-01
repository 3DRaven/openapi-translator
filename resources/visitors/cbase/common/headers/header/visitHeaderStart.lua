--- This visitor is invoked before processing response header
--- @param headerName string|null #
--- @param header Header #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitHeaderStart(headerName, header, extensions, callId)
    return {}
end

return functionCallAndLog("visitHeaderStart", visitHeaderStart, 1)
