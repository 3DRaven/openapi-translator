--- This visitor is invoked before processing example in response header examples if reference
--- @param exampleName string #
--- @param exampleReference string           #
--- @param extensions table             # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[]            # Returns the output code and  file name for writing code
local function visitExampleReference(exampleName, exampleReference, extensions, callId)
    return {}
end

return functionCallAndLog("visitExampleReference", visitExampleReference)
