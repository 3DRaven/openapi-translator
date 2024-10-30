--- This visitor is invoked after processing response header example format media type
--- @param mediaTypeName string #
--- @param mediaType MediaType # a media type with potentially multiple examples and encoding information
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitMediaTypeEnd(mediaTypeName, mediaType, extensions, callsStack)
    global_context.names:pop()
    return {}
end

return functionCallAndLog("visitMediaTypeEnd", visitMediaTypeEnd)
