--- This visitor is invoked at the start of OpenAPI scpec after processing tags on by one
--- @param tags Tag[] # OpenAPI described servers
--- @param extensions table<string,any> # table with free form with "x-" OpenAPI extensions for this level of spec (root level)
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSpecTagsEnd(tags, extensions, callId)
    return {}
end

return functionCallAndLog("visitSpecTagsEnd", visitSpecTagsEnd, -1)
