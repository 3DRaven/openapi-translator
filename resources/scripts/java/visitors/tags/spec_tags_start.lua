--- @class TagDescriptor
--- Adds metadata to a single tag used by the Operation Object.
--- It is not mandatory to have a Tag Object per tag defined in the Operation Object instances.
--- @field name string # REQUIRED. The name of the tag.
--- @field description string|nil # A short description for the tag. CommonMark syntax MAY be used.
--- @field external_docs ExternalDocsDescriptor|nil # Additional external documentation for this tag.
--- @field extensions table<string, any> # Inline extensions to this object.

--- This visitor is invoked at the start of OpenAPI scpec before processing tags on by one
--- @param tags TagDescriptor[] # OpenAPI described servers
--- @param extensions table<string,any> # table with free form with "x-" OpenAPI extensions for this level of spec (root level)
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecTagsStart(tags, extensions)
    return {}
end

return functionCallAndLog("visitSpecTagsStart", visitSpecTagsStart)
