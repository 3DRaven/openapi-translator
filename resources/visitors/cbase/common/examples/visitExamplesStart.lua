--- This visitor is invoked before processing all response header examples
--- @param examples table<string, Example> # Represents the headers parameter, which is a map from strings to references or items.
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitExamplesStart(examples, extensions, callId)
    return {}
end

return functionCallAndLog("visitExamplesStart", visitExamplesStart, 1)
