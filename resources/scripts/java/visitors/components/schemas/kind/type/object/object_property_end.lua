--- This visitor is invoked after processing the found property within a schema of type object is completed.
--- If this property contains, for example, another object, it has already been processed.
--- Returns a code of the end of an object property code.
--- This visitor called after generate property code itself
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param object_description ObjectDescriptor # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitObjectPropertyEnd(namesStack, object_description, extensions)
    return {}
end

local function afterDecorator()
    global_context:dropLastChildrenModelName("visitObjectPropertyEnd")
end

return functionCallAndLog("visitObjectPropertyEnd", visitObjectPropertyEnd, nil, afterDecorator)
