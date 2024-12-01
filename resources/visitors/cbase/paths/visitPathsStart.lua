--- This visitor is invoked at start of processing paths in spec
--- @param paths Paths #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitPathsStart(paths, extensions, callId)
    return {}
end

return functionCallAndLog("visitPathsStart", visitPathsStart, 1)
