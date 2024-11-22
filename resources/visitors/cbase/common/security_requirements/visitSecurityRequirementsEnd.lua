--- This visitor is invoked at the end of processing security schemes on by one
--- @param securities table<string,string[]>[] # OpenAPI described security schemes
--- @param extensions table<string,any> # table with free form with "x-" OpenAPI extensions for this level of spec (root level)
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecurityRequirementsEnd(securities, extensions, callId)
    return {}
end

return functionCallAndLog("visitSecurityRequirementsEnd", visitSecurityRequirementsEnd)
