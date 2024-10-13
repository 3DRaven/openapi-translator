--- Represents schema data.
---@class SchemaDescriptor
---@field nullable boolean # Indicates if the schema is nullable
---@field readOnly boolean # Indicates if the schema is read-only
---@field writeOnly boolean # Indicates if the schema is write-only
---@field deprecated boolean # Indicates if the schema is deprecated
---@field externalDocs ExternalDocsDescriptor | nil # Reference to external documentation
---@field example any | nil # An example of the schema
---@field title string | nil # The title of the schema
---@field description string | nil # A description of the schema
---@field discriminator DiscriminatorDescriptor | nil # Discriminator for the schema
---@field default any | nil # Default value of the schema
---@field extensions table<string, any> # Inline extensions to this object

--- This visitor is invoked before processing any kind of schema
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param schemaDescriptor SchemaDescriptor # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSchemaStart(namesStack, schemaDescriptor, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSchemaStart", visitSchemaStart)
