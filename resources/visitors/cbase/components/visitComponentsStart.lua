--- This visitor is invoked before processing components
--- @param components Components #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitComponentsStart(components, extensions, callId)
    return {}
end

return functionCallAndLog("visitComponentsStart", visitComponentsStart)
