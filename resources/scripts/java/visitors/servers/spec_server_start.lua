--- @class ServerVariable
--- An object representing a Server Variable for server URL template substitution.
--- @field enumeration string[] # An enumeration of string values for limited set substitution options.
--- @field default string # REQUIRED. The default value to use for substitution if an alternate is not supplied.
--- @field description string|nil # An optional description for the server variable.
--- @field extensions table<string, any> # Inline extensions to this object.

--- This visitor is invoked before of OpenAPI scpec for every described server
--- @param url string # REQUIRED. A URL to the target host. Supports Server Variables and MAY be relative.
--- @param description string|nil # An optional string describing the host designated by the URL.
--- @param variables table<string, ServerVariable>|nil # A map between a variable name and its value for URL template substitution.
--- @param callStack table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table<string, any> # Inline extensions to this object.
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecServerStart(url, description, variables, callStack, extensions)
    return {}
end

return functionCallAndLog("visitSpecServerStart", visitSpecServerStart)
