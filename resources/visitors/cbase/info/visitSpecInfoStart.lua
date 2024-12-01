--- This visitor is invoked before of OpenAPI scpec in info section
--- @param info Info # OpenAPI version
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSpecInfoStart(info, extensions, callId)
    return {}
end

return functionCallAndLog("visitSpecInfoStart", visitSpecInfoStart, 1)
