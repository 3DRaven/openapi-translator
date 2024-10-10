--- @class Contact
--- Contact information for the exposed API.
--- @field name string|nil # The identifying name of the contact person/organization.
--- @field url string|nil # The URL pointing to the contact information. MUST be in the format of a URL.
--- @field email string|nil # The email address of the contact person/organization. MUST be in the format of an email address.
--- @field extensions table<string, any> # Inline extensions to this object.

--- This visitor is invoked at the start of OpenAPI scpec with contact in info section if it exists
--- @param contact Contact # License information
--- @param extensions table<string, any> # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecInfoContact(contact, extensions)
    return {}
end

return functionCallAndLog("visitSpecInfoContact", visitSpecInfoContact)
