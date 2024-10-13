--- The object provides metadata about the API.
--- The metadata MAY be used by the clients if needed,
--- and MAY be presented in editing or documentation generation tools for convenience.
--- @class Info
--- @field title string # REQUIRED. The title of the application.
--- @field description string|nil # A short description of the application. CommonMark syntax MAY be used for rich text representation.
--- @field terms_of_service string|nil # A URL to the Terms of Service for the API. MUST be in the format of a URL.
--- @field contact Contact|nil # The contact information for the exposed API.
--- @field license License|nil # The license information for the exposed API.
--- @field version string # REQUIRED. The version of the OpenAPI document, distinct from the OpenAPI Specification version or the API implementation version.
--- @field extensions table<string, any> # Inline extensions to this object.

--- This visitor is invoked before of OpenAPI scpec in info section
--- @param info Info # OpenAPI version
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecInfoStart(info, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSpecInfoStart", visitSpecInfoStart)
