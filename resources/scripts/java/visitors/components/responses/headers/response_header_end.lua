--- This visitor is invoked after processing response header
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param header Header # Represents the header parameter
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitResponseHeaderEnd(namesStack, header, extensions)
    return {}
end

return functionCallAndLog("visitResponseHeaderEnd", visitResponseHeaderEnd)
