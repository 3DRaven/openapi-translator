--- This visitor is invoked at end of processing paths in spec
--- @param paths Paths #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitPathsEnd(paths, extensions, callId)
    return {}
end

return functionCallAndLog("visitPathsEnd", visitPathsEnd)
