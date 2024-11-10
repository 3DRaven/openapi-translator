--- This visitor is invoked at the start of processing security schemes on by one
--- @param securities table<string,string[]>[] # OpenAPI described security schemes
--- @param extensions table<string,any> # table with free form with "x-" OpenAPI extensions for this level of spec (root level)
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecurityRequirementsStart(securities, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSecurityRequirementsStart", visitSecurityRequirementsStart)
