--- This visitor handles the processing before property of object schema.
--- @param propertyName string|null #
--- @param schema Schema # free form of additionalProperties has this value
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitObjectPropertyStart(propertyName, schema, extensions, callId)
    --- @type ModelBase
    local model = GLOBAL_CONTEXT.models:element()
    model:addModelProperty(nullableAsNillable(propertyName), extensions)
    return {}
end

return functionCallAndLog("visitObjectPropertyStart", visitObjectPropertyStart)
