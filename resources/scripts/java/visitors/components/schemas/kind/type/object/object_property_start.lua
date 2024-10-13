--- This visitor handles the processing before property of object schema.
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param flag boolean # free form of additionalProperties has this value
--- @param minProperties integer? # minimal number of properties in additionalProperties collection
--- @param maxProperties integer? # maximal number of properties in additionalProperties collection
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitObjectPropertyStart(namesStack, flag, minProperties, maxProperties, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitObjectPropertyStart", visitObjectPropertyStart)
