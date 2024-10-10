--- This visitor is invoked at the start of OpenAPI scpec after processing tags on by one
--- @param tags TagDescriptor[] # OpenAPI described servers
--- @param extensions table<string,any> # table with free form with "x-" OpenAPI extensions for this level of spec (root level)
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecTagsEnd(tags, extensions)
    return {}
end

return functionCallAndLog("visitSpecTagsEnd", visitSpecTagsEnd)
