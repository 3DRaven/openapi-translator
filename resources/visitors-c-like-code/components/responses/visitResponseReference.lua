--- This visitor is invoked before processing response if reference
--- @param responseName string|null #
--- @param responseReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitResponseReference(responseName, responseReference, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitResponseReference", visitResponseReference)
