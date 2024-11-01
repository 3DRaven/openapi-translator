--- This visitor handles the processing before property of object schema.
--- @param propertyName string #
--- @param schema Schema # free form of additionalProperties has this value
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitObjectPropertyStart(propertyName, schema, extensions, callsStack)
    --- @type ModelBase
    local model = GLOBAL_CONTEXT.models:element()
    model:addModelProperty(propertyName, extensions)
    return {}
end

return functionCallAndLog("visitObjectPropertyStart", visitObjectPropertyStart)
