--- This visitor is invoked after processing anyOf element
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param required boolean # Indicates if the property value (this object) is required
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitAnyOfEnd(namesStack, required, extensions, callsStack)
    return {}
end

local function beforeDecorator(namesStack)
    -- drop before main code because we need to know parent for this object if it exists, this object not a parent now
    global_context:dropLastParentType("visitAnyOfEnd")
end

return functionCallAndLog("visitAnyOfEnd", visitAnyOfEnd, beforeDecorator)
