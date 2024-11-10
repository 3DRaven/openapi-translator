--- This visitor is invoked after processing response
--- @param responseName string|null #
--- @param response Response # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitResponseEnd(responseName, response, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitResponseEnd", visitResponseEnd)
