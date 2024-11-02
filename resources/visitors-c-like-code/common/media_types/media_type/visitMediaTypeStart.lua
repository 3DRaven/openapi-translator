--- This visitor is invoked before processing response header example format media type
--- @param mediaTypeName string #
--- @param mediaType MediaType # a media type with potentially multiple examples and encoding information
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitMediaTypeStart(mediaTypeName, mediaType, extensions, callsStack)
    -- here we can have application/json or an extended model name
    local name = extensions[Extensions.MODEL_NAME]
    if name then
        GLOBAL_CONTEXT.names:push(name)
    end
    return {}
end

return functionCallAndLog("visitMediaTypeStart", visitMediaTypeStart)