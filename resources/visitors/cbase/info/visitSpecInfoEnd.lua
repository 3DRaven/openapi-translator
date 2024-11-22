--- This visitor is invoked after of OpenAPI scpec in info section
--- @param info Info # info section
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSpecInfoEnd(info, extensions, callId)
    return {}
end

return functionCallAndLog("visitSpecInfoEnd", visitSpecInfoEnd)
