--- Represents a property of type boolean.
---@class BooleanType
---@field enumeration (boolean | nil)[] # The enumeration of possible boolean values. Can contain true, false, or nil values.

--- This visitor is invoked when a property of type boolean is found.
--- Returns a code for creating storage for additionalProperties (Map as example)
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param booleanDescriptor BooleanType # boolean property descriptor
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitBooleanProperty(namesStack, booleanDescriptor, extensions, callsStack)
    local parentModelName = getParentModelName(namesStack)
    if parentModelName == nil then
        --- This is possible if a schema is constructed as a separate value;
        --- such a schema is not a model but can be used as a reference in other schemas.
        --- For example, you can add descriptions and other elements to it instead of copying them
        --- within the specification. So, we simply don't need to do anything in this case.
        print("Boolean property without parent skipt")
    else
        local parentType = global_context:getLastParentType("visitBooleanProperty")
        if parentType == ParentType.OBJECT or
            parentType == ParentType.ALL_OF then
            local currentPropertyName = getCurrentPropertyNameMandatory(namesStack)
            local required = global_context:isPropertyRequired("visitIntegerProperty", parentModelName,
                currentPropertyName)
            local requiredMarker = getRequiredMarker(required, "@NonNull ")

            local code = string.format("    private %sBoolean %s;\n", requiredMarker,
                currentPropertyName);

            global_context:addProperties("visitBooleanProperty", parentModelName,
                { WriteOperation.new_append(code, parentModelName) })
        elseif parentType == ParentType.ARRAY then
        elseif parentType == ParentType.ADDITIONAL then
        else
            error("Unknown parent type for Boolean")
        end
    end
    return {}
end

local function beforeDecorator()
    global_context:addLastChildrenModelName("visitBooleanProperty", "Boolean")
end

return functionCallAndLog("visitBooleanProperty", visitBooleanProperty, beforeDecorator)
