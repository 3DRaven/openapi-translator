--- This visitor is called before the schema in the not property is processed
--- @param schema ReferenceOr<Schema> # chain of model names from root to this point
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitPropertyNotStart(schema, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitPropertyNotStart", visitPropertyNotStart)
