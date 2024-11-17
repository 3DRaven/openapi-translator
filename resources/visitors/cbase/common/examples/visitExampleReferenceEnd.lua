--- This visitor is invoked after processing example in response header examples if reference
--- @param exampleName string #
--- @param exampleReference string           #
--- @param extensions table             # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[]            # Returns the output code and  file name for writing code
local function visitExampleReferenceEnd(exampleName, exampleReference, extensions, callId)
    return {}
end

return functionCallAndLog("visitExampleReferenceEnd", visitExampleReferenceEnd)
