--- This visitor is invoked at the start of OpenAPI scpec for every described tag
--- @param name string # REQUIRED. The name of the tag.
--- @param description string|nil # A short description for the tag. CommonMark syntax MAY be used.
--- @param external_docs ExternalDocsDescriptor|nil # Additional external documentation for this tag.
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecTag(name, description, external_docs, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSpecTag", visitSpecTag)
