--- This visitor is invoked after processing links
--- @param links table<string, ReferenceOr<Link>>
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitLinksEnd(links, extensions, callId)
    return {}
end

return functionCallAndLog("visitLinksEnd", visitLinksEnd, -1)
