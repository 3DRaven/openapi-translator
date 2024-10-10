--- This visitor is invoked before processing the found schema in oneOf element
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param required boolean # Indicates if the property value (this object) is required
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitOneOfEnd(namesStack, required, extensions)
    return {}
end

local function beforeDecorator(namesStack)
    -- drop before main code because we need to know parent for this object if it exists, this object not a parent now
    global_context:dropLastParentType("visitOneOfEnd")
end

return functionCallAndLog("visitOneOfEnd", visitOneOfEnd, beforeDecorator)
