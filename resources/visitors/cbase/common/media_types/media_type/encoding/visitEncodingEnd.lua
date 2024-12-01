--- This visitor is invoked after processing response header example format media type encoding
--- @param encodingName string #
--- @param encoding Encoding                # Content-Type for encoding a specific property.
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitEncodingEnd(encodingName, encoding, extensions, callId)
    return {}
end

return functionCallAndLog("visitEncodingEnd", visitEncodingEnd, -1)
