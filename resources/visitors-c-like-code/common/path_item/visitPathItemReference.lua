--- This visitor is invoked before processing path item if reference
--- @param pathItemName string|null
--- @param pathItemReference string
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitPathItemReference(pathItemName, pathItemReference, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitPathItemReference", visitPathItemReference)
