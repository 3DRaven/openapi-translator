--- This visitor is invoked before processing response header
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param header Header # Represents the header parameter
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitResponseHeaderStart(namesStack, header, extensions)
    return {}
end

return functionCallAndLog("visitResponseHeaderStart", visitResponseHeaderStart)
