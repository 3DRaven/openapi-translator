--- Allows referencing an external resource for extended documentation.
---@class ExternalDocsDescriptor
---@field description string | nil # A short description of the target documentation
---@field url string # REQUIRED. URL for the target documentation
---@field extensions table<string, any> # Inline extensions to this object

--- This visitor is invoked at externalDocs of schema
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param externalDocsDescriptor ExternalDocsDescriptor # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecExternalDocs(namesStack, externalDocsDescriptor, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSpecExternalDocs", visitSpecExternalDocs)
