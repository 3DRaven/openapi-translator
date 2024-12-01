--- This visitor is invoked before processing path item
--- @param pathItemName string|null
--- @param pathItem PathItem
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitPathItemStart(pathItemName, pathItem, extensions, callId)
    return {}
end

return functionCallAndLog("visitPathItemStart", visitPathItemStart, 1)
