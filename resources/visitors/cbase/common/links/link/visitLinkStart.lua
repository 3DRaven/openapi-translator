--- This visitor is invoked before processing link
--- @param linkName string|null #
--- @param link Link #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitLinkStart(linkName, link, extensions, callId)
    return {}
end

return functionCallAndLog("visitLinkStart", visitLinkStart)
