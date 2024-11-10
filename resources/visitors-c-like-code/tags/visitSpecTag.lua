--- This visitor is invoked at the start of OpenAPI scpec for every described tag
--- @param tag Tag #
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSpecTag(tag, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSpecTag", visitSpecTag)
