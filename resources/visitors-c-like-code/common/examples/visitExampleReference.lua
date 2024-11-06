--- This visitor is invoked before processing example in response header examples if reference
--- @param exampleName string #
--- @param exampleReference string           #
--- @param extensions table             # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[]            # Returns the output code and  file name for writing code
local function visitExampleReference(exampleName, exampleReference, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitExampleReference", visitExampleReference)
