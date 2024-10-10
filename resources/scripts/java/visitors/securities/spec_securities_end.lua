--- This visitor is invoked at the start of OpenAPI scpec after processing security schemes on by one
--- @param securities table<string,string[]>[] # OpenAPI described security schemes
--- @param extensions table<string,any> # table with free form with "x-" OpenAPI extensions for this level of spec (root level)
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecSecuritiesEnd(securities, extensions)
    return {}
end

return functionCallAndLog("visitSpecSecuritiesEnd", visitSpecSecuritiesEnd)
