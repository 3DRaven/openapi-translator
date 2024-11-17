--- This visitor is invoked after described server
--- @param server Server # REQUIRED. A URL to the target host. Supports Server Variables and MAY be relative.
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitServerEnd(server, extensions, callId)
    return {}
end

return functionCallAndLog("visitServerEnd", visitServerEnd)