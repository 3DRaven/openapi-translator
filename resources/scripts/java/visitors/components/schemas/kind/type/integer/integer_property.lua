--- This visitor is invoked when a property of type integer is found.
--- Returns a code for creating integer value property of object
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param integerDescriptor IntegerType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitIntegerProperty(namesStack, integerDescriptor, extensions, callsStack)
    local parentModelName = getParentModelName(namesStack)
    if parentModelName == nil then
        --- This is possible if a schema is constructed as a separate value;
        --- such a schema is not a model but can be used as a reference in other schemas.
        --- For example, you can add descriptions and other elements to it instead of copying them
        --- within the specification. So, we simply don't need to do anything in this case.
        print("Integer property without parent skipt")
    else
        local parentType = global_context:getLastParentType("visitIntegerProperty")
        if parentType == ParentType.OBJECT or
            parentType == ParentType.ALL_OF then
            local currentPropertyName = getCurrentPropertyNameMandatory(namesStack)
            local required = global_context:isPropertyRequired("visitIntegerProperty", parentModelName,
                currentPropertyName)
            local requiredMarker = getRequiredMarker(required, "@NonNull ")

            local code = string.format("    private %sInteger %s;\n", requiredMarker,
                currentPropertyName);

            global_context:addProperties("visitIntegerProperty", parentModelName,
                { WriteOperation.new_append(code, parentModelName) })
        elseif parentType == ParentType.ARRAY then
        elseif parentType == ParentType.ADDITIONAL then
        else
            error("Unknown parent type for Integer")
        end
    end
    return {}
end

local function beforeDecorator()
    global_context:addLastChildrenModelName("visitIntegerProperty", "Integer")
end

return functionCallAndLog("visitIntegerProperty", visitIntegerProperty, beforeDecorator)
