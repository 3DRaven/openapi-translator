--- This visitor is invoked at the start of OpenAPI scpec in inf section if it exists
--- @param license License # License information
--- @param extensions table<string, any> # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecInfoLicense(license, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSpecInfoLicense", visitSpecInfoLicense)
