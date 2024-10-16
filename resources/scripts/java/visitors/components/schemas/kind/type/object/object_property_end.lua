--- This visitor handles the processing of object schema property.
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param schema Schema # free form of additionalProperties has this value
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitObjectPropertyEnd(namesStack, schema, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitObjectPropertyEnd", visitObjectPropertyEnd)
