--- This visitor is invoked before processing response header example format media type encoding
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param encoding Encoding                # Content-Type for encoding a specific property.
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitEncodingStart(namesStack, encoding, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitEncodingStart", visitEncodingStart)
