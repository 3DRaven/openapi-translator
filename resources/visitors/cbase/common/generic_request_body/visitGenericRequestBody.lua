--- This visitor is invoked for processing generic json request body
--- @param body table # Represents the media type example
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitGenericRequestBody(body, extensions, callId)
    return {}
end

return functionCallAndLog("visitGenericRequestBody", visitGenericRequestBody)
