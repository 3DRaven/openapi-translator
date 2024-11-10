--- This visitor is invoked at the start of OpenAPI scpec with contact in info section if it exists
--- @param contact Contact # License information
--- @param extensions table<string, any> # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSpecInfoContact(contact, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSpecInfoContact", visitSpecInfoContact)
