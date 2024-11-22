--- This visitor is invoked at the start of OpenAPI scpec in inf section if it exists
--- @param license License # License information
--- @param extensions table<string, any> # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSpecInfoLicense(license, extensions, callId)
    return {}
end

return functionCallAndLog("visitSpecInfoLicense", visitSpecInfoLicense)
