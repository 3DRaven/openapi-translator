--- This visitor is invoked after processing security scheme
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param securityScheme table # generic security scheme
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSecuritySchemeStart(namesStack, securityScheme, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSecuritySchemeStart", visitSecuritySchemeStart)
