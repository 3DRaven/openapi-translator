--- This visitor is invoked at the start of OpenAPI scpec for every described tag
--- @param tag Tag #
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSpecTag(tag, extensions, callId)
    return {}
end

return functionCallAndLog("visitSpecTag", visitSpecTag)
