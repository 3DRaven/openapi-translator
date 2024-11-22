--- This visitor is invoked for processing generic json example
--- @param example table # Represents the media type example
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitGenericExample(example, extensions, callId)
    return {}
end

return functionCallAndLog("visitGenericExample", visitGenericExample)
