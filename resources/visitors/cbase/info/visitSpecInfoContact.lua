--- This visitor is invoked at the start of OpenAPI scpec with contact in info section if it exists
--- @param contact Contact # License information
--- @param extensions table<string, any> # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSpecInfoContact(contact, extensions, callId)
    return {}
end

return functionCallAndLog("visitSpecInfoContact", visitSpecInfoContact)
