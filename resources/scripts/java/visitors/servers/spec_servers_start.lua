--- @class Server
--- An object representing a Server.
--- @field url string # REQUIRED. A URL to the target host. Supports Server Variables and MAY be relative.
--- @field description string|nil # An optional string describing the host designated by the URL.
--- @field variables table<string, ServerVariable>|nil # A map between a variable name and its value for URL template substitution.
--- @field extensions table<string, any> # Inline extensions to this object.

--- This visitor is invoked at the start of OpenAPI scpec before processing servers on by one
--- @param servers Server[] # OpenAPI described servers
--- @param extensions table<string,any> # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecServersStart(servers, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSpecServersStart", visitSpecServersStart)
