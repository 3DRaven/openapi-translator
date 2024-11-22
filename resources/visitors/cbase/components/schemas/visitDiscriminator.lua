--- This visitor is invoked at discriminator in schema
--- @param discriminatorDescriptor Discriminator # descriptor of discriminator
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitDiscriminator(discriminatorDescriptor, extensions, callId)
    return {}
end

return functionCallAndLog("visitDiscriminator", visitDiscriminator)
