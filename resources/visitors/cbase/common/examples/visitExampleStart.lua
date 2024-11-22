--- This visitor is invoked before processing example in response header examples
--- @param exampleName string|null #
--- @param example Example           # Short description for the example.
--- @param extensions table             # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[]            # Returns the output code and  file name for writing code
local function visitExampleStart(exampleName, example, extensions, callId)
    return {}
end

return functionCallAndLog("visitExampleStart", visitExampleStart)
