--- This visitor is invoked before processing example in response header examples
--- @param exampleName string #
--- @param example Example           # Short description for the example.
--- @param extensions table             # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[]            # Returns the output code and  file name for writing code
function visitExampleStart(exampleName, example, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitExampleStart", visitExampleStart)
