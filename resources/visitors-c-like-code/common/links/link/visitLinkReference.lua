--- This visitor is invoked before processing link reference
--- @param linkName string|null #
--- @param linkReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitLinkReference(linkName, linkReference, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitLinkReference", visitLinkReference)
