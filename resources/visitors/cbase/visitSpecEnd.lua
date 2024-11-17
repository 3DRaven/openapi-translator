--- This visitor is invoked at the end of OpenAPI scpec
--- @param version string # OpenAPI version
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSpecEnd(version, extensions, callId)
    print("Spec processing finished")
    printCalls()
    return {}
end

return functionCallAndLog("visitSpecEnd", visitSpecEnd)
