--- This visitor is invoked after processing response header example format media type
--- @param mediaTypeName string #
--- @param mediaType MediaType # a media type with potentially multiple examples and encoding information
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitMediaTypeEnd(mediaTypeName, mediaType, extensions, callId)
    return {}
end

return functionCallAndLog("visitMediaTypeEnd", visitMediaTypeEnd)
