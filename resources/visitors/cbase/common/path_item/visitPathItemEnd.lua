--- This visitor is invoked after processing path item
--- @param pathItemName string|null
--- @param pathItem PathItem
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitPathItemEnd(pathItemName, pathItem, extensions, callId)
    return {}
end

return functionCallAndLog("visitPathItemEnd", visitPathItemEnd)
