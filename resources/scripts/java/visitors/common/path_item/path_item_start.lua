--- This visitor is invoked before processing path item
--- @param pathItemName string
--- @param pathItem PathItem
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitPathItemStart(pathItemName, pathItem, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitPathItemStart", visitPathItemStart)
