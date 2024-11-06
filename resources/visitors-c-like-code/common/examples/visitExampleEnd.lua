--- This visitor is invoked after processing example in response header examples
--- @param exampleName string|null #
--- @param example Example           # Short description for the example.
--- @param extensions table             # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[]            # Returns the output code and  file name for writing code
function visitExampleEnd(exampleName, example, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitExampleEnd", visitExampleEnd)
