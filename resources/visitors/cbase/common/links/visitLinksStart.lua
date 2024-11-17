--- This visitor is invoked before processing links
--- @param links table<string, ReferenceOr<Link>>
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitLinksStart(links, extensions, callId)
    return {}
end

return functionCallAndLog("visitLinksStart", visitLinksStart)