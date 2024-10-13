--- This visitor is invoked to process a found schema without a defined structure,
--- such as `additionalProperties: {}`.
--- Returns a model name with unknown structure (it is {} in OpenAPI)
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param required boolean # Indicates if the property value (this object) is required
--- @param anySchemaDescriptor AnySchema # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitAnySchema(namesStack, required, anySchemaDescriptor, extensions, callsStack)
    return {}
end

local function beforeDecorator()
    global_context:addLastChildrenModelName("visitAnySchema", "Object")
end

return functionCallAndLog("visitAnySchema", visitAnySchema, beforeDecorator)
