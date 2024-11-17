--- This visitor is invoked before processing path item if reference
--- @param pathItemName string|null
--- @param pathItemReference string
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitPathItemReferenceStart(pathItemName, pathItemReference, extensions, callId)
    return {}
end

return functionCallAndLog("visitPathItemReferenceStart", visitPathItemReferenceStart)
