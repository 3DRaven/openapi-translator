--- This visitor is invoked after processing response header example format media type
--- @param mediaTypeName string #
--- @param mediaType MediaType # a media type with potentially multiple examples and encoding information
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitMediaTypeEnd(mediaTypeName, mediaType, extensions, callsStack)
    -- here we can have application/json or an extended model name
    if extensions[Extensions.MODEL_NAME] then
        GLOBAL_CONTEXT.names:pop()
    end
    return {}
end

return functionCallAndLog("visitMediaTypeEnd", visitMediaTypeEnd)
