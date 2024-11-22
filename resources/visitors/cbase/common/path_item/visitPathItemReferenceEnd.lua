--- This visitor is invoked after processing path item if reference
--- @param pathItemName string|null
--- @param pathItemReference string
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitPathItemReferenceEnd(pathItemName, pathItemReference, extensions, callId)
    return {}
end

return functionCallAndLog("visitPathItemReferenceEnd", visitPathItemReferenceEnd)
