--- This visitor handles the processing after object schema properties.
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param properties table<string,Schema> # free form of additionalProperties has this value
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitObjectPropertiesEnd(namesStack, properties, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitObjectPropertiesEnd", visitObjectPropertiesEnd)
