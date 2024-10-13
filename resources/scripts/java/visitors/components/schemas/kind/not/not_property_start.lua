--- This visitor is called before the schema in the not property is processed
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param required boolean # Indicates if the property value (this object) is required
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitPropertyNotStart(namesStack, required, extensions, callsStack)
    return {}
end

local function beforeDecorator(namesStack)
    global_context:addParentType("visitPropertyNotStart", ParentType.NOT)
end

return functionCallAndLog("visitPropertyNotStart", visitPropertyNotStart, beforeDecorator)
