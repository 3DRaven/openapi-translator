--- This visitor is invoked at default value of schema
--- @param default table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitDefault(default, extensions, callId)
    return {}
end

return functionCallAndLog("visitDefault", visitDefault)
