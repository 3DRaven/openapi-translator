--- This visitor is invoked at discriminator in schema
--- @param discriminatorDescriptor Discriminator # descriptor of discriminator
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitDiscriminator(discriminatorDescriptor, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitDiscriminator", visitDiscriminator)
