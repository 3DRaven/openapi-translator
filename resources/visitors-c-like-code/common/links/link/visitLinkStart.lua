--- This visitor is invoked before processing link
--- @param linkName string|null #
--- @param link Link #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitLinkStart(linkName, link, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitLinkStart", visitLinkStart)
