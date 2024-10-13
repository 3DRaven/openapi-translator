--- This visitor is invoked at the start of OpenAPI scpec for processing security schemas on by one
--- @param security table<string,string[]> # OpenAPI described security schema
--- @param extensions table<string,any> # table with free form with "x-" OpenAPI extensions for this level of spec (root level)
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecSecurity(security, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSpecSecurity", visitSpecSecurity)
