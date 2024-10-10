--- This visitor is invoked at the start of OpenAPI scpec after processing servers on by one
--- @param servers Server[] # OpenAPI described servers
--- @param extensions table<string,any> # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecServersEnd(servers, extensions)
    return {}
end

return functionCallAndLog("visitSpecServersEnd", visitSpecServersEnd)
