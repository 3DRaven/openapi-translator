--- This visitor is invoked after of OpenAPI scpec for every described server
--- @param url string # REQUIRED. A URL to the target host. Supports Server Variables and MAY be relative.
--- @param description string|nil # An optional string describing the host designated by the URL.
--- @param variables table<string, ServerVariable>|nil # A map between a variable name and its value for URL template substitution.
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecServerEnd(url, description, variables, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSpecServerEnd", visitSpecServerEnd)
