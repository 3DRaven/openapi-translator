--- This visitor is invoked at the end of servers collection
--- @param servers Server[] # OpenAPI described servers
--- @param extensions table<string,any> # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitServersEnd(servers, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitServersEnd", visitServersEnd)
