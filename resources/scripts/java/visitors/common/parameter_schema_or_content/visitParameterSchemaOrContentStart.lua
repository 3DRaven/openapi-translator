--- This visitor is invoked before processing response header example format
--- @param parameterName string|null #
--- @param format ParameterSchemaOrContent # Represents the schema or content representation for a parameter
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitParameterSchemaOrContentStart(parameterName, format, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitParameterSchemaOrContentStart", visitParameterSchemaOrContentStart)
