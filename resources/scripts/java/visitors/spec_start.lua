--- This visitor is invoked at the start of OpenAPI scpec
--- @param version string # OpenAPI version
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecStart(version, extensions)
    return {}
end

return functionCallAndLog("visitSpecStart", visitSpecStart)
