--- This visitor is invoked for processing security scheme
--- @param securities table<string,string[]> # OpenAPI described security schemes
--- @param extensions table<string,any> # table with free form with "x-" OpenAPI extensions for this level of spec (root level)
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecurityRequirement(securities, extensions, callId)
    return {}
end

return functionCallAndLog("visitSecurityRequirement", visitSecurityRequirement)
