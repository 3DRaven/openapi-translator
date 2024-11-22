--- This visitor is invoked at the end of servers collection
--- @param servers Server[] # OpenAPI described servers
--- @param extensions table<string,any> # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitServersEnd(servers, extensions, callId)
    return {}
end

return functionCallAndLog("visitServersEnd", visitServersEnd)
