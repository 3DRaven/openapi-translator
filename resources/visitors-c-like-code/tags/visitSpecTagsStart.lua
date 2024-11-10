--- This visitor is invoked at the start of OpenAPI scpec before processing tags on by one
--- @param tags Tag[] # OpenAPI described servers
--- @param extensions table<string,any> # table with free form with "x-" OpenAPI extensions for this level of spec (root level)
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSpecTagsStart(tags, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSpecTagsStart", visitSpecTagsStart)
