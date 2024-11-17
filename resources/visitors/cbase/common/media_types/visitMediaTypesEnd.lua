--- This visitor is invoked after processing media types
--- @param mediaTypes table<string, MediaType>
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitMediaTypesEnd(mediaTypes, content, extensions, callId)
    return {}
end

return functionCallAndLog("visitMediaTypesEnd", visitMediaTypesEnd)
