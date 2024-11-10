--- This visitor is invoked after processing media type encodings
--- @param encodings table<string,Encoding> # encodings information
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitMediaTypeEncodingsEnd(encodings, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitMediaTypeEncodingsEnd", visitMediaTypeEncodingsEnd)
