--- This visitor is invoked at the start of OpenAPI scpec
--- @param version string # OpenAPI version
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSpecStart(version, extensions, callId)
    return {}
end

return functionCallAndLog("visitSpecStart", visitSpecStart, 1)
