--- This visitor is invoked after processing response header example format
--- @param parameterName string|null #
--- @param format ParameterSchemaOrContent # Represents the schema or content representation for a parameter
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitParameterSchemaOrContentEnd(parameterName, format, extensions, callId)
    return {}
end

return functionCallAndLog("visitParameterSchemaOrContentEnd", visitParameterSchemaOrContentEnd)
