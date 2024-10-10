--- This visitor is invoked before processing a found property inside a schema of type object.
--- Returns a code of the start of an object property code.
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param object_description ObjectDescriptor # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitObjectPropertyStart(namesStack, object_description, extensions)
    
    return {}
end

return functionCallAndLog("visitObjectPropertyStart", visitObjectPropertyStart)
