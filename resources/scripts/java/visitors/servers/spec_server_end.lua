--- This visitor is invoked after of OpenAPI scpec for every described server
--- @param url string # REQUIRED. A URL to the target host. Supports Server Variables and MAY be relative.
--- @param description string|nil # An optional string describing the host designated by the URL.
--- @param variables table<string, ServerVariable>|nil # A map between a variable name and its value for URL template substitution.
--- @param callStack table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table<string, any> # Inline extensions to this object.
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecServerEnd(url, description, variables, callStack, extensions)
    return {}
end

return functionCallAndLog("visitSpecServerEnd", visitSpecServerEnd)
