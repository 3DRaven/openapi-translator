--- This visitor is invoked after processing media type encodings
--- @param encodings table<string,Encoding> # encodings information
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitMediaTypeEncodingsEnd(encodings, extensions, callId)
    return {}
end

return functionCallAndLog("visitMediaTypeEncodingsEnd", visitMediaTypeEncodingsEnd, -1)
