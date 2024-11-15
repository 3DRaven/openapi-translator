--- This visitor is invoked at the start of servers collection
--- @param servers Server[] # OpenAPI described servers
--- @param extensions table<string,any> # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitServersStart(servers, extensions, callId)
    return {}
end

return functionCallAndLog("visitServersStart", visitServersStart)
