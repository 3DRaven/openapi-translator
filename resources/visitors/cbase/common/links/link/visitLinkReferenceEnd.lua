--- This visitor is invoked after processing link reference
--- @param linkName string|null #
--- @param linkReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitLinkReferenceEnd(linkName, linkReference, extensions, callId)
    return {}
end

return functionCallAndLog("visitLinkReferenceEnd", visitLinkReferenceEnd)
