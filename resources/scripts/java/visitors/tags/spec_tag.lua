--- Represents an ExternalDocumentation Object that allows referencing an external resource for extended documentation.
---@class ExternalDocumentation
---@field description string | nil # A short description of the target documentation. Supports CommonMark syntax for rich text representation.
---@field url string # The URL for the target documentation (Required). Must be a valid URL format.
---@field extensions table<string, any> # Inline extensions to this object.

--- Represents a Tag Object which adds metadata to a single tag used by the Operation Object.
---@class Tag
---@field name string # The name of the tag (Required).
---@field description string | nil # A short description for the tag. Supports CommonMark syntax for rich text representation.
---@field external_docs ExternalDocumentation | nil # Additional external documentation for this tag.
---@field extensions table<string, any> # Inline extensions to this object.

--- This visitor is invoked at the start of OpenAPI scpec for every described tag
--- @param tag Tag #
--- @param extensions table<string, any> # Inline extensions to this object.
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecTag(tag, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSpecTag", visitSpecTag)
