--- This visitor is invoked before processing response header example format media type encoding
--- @param encodingName string #
--- @param encoding Encoding                # Content-Type for encoding a specific property.
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitEncodingStart(encodingName, encoding, extensions, callId)
    return {}
end

return functionCallAndLog("visitEncodingStart", visitEncodingStart)
