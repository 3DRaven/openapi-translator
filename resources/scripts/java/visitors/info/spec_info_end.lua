--- This visitor is invoked after of OpenAPI scpec in info section
--- @param info Info # info section
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecInfoEnd(info, callsStack, extensions)
    return {}
end

return functionCallAndLog("visitSpecInfoEnd", visitSpecInfoEnd)
