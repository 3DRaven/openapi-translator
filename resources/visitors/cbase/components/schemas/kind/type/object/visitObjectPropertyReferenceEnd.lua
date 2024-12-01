--- This visitor handles the processing after property of object schema if reference
--- @param propertyName string|null #
--- @param schemaReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitObjectPropertyReferenceEnd(propertyName, schemaReference, extensions, callId)
    return {}
end

return functionCallAndLog("visitObjectPropertyReferenceEnd", visitObjectPropertyReferenceEnd, -1)
