--- Used to aid in serialization, deserialization, and validation when request bodies or response payloads
--- may be one of a number of different schemas.
---@class Discriminator
---@field propertyName string # REQUIRED. Name of the property in the payload holding the discriminator value
---@field mapping table<string, string> # Mappings between payload values and schema names or references
---@field extensions table<string, any> # Inline extensions to this object

--- This visitor is invoked at discriminator in schema
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param discriminatorDescriptor Discriminator # descriptor of discriminator
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitDiscriminator(namesStack, discriminatorDescriptor, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitDiscriminator", visitDiscriminator)
