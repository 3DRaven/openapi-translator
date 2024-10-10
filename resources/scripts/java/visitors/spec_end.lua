--- This visitor is invoked at the end of OpenAPI scpec
--- @param version string # OpenAPI version
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecEnd(version, extensions)
    return {}
end

return functionCallAndLog("visitSpecEnd", visitSpecEnd)
