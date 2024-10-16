--- This visitor is invoked when a property of type number is found.
--- Returns a code for creating property for number value
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param numberDescriptor StringType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitNumberProperty(namesStack, numberDescriptor, extensions, callsStack)
    local parentModelName = getParentModelName(namesStack)
    if parentModelName == nil then
        --- This is possible if a schema is constructed as a separate value;
        --- such a schema is not a model but can be used as a reference in other schemas.
        --- For example, you can add descriptions and other elements to it instead of copying them
        --- within the specification. So, we simply don't need to do anything in this case.
        print("Number property without parent skipt")
    else
        if hasSpecifiedParentsInCallChain("visitNumberProperty",
                callsStack, { Script.OBJECT_START, Script.ALL_OF_START }) then
            local currentPropertyName = getCurrentPropertyNameMandatory(namesStack)
            local required = global_context:isPropertyRequired("visitNumberProperty", parentModelName,
                currentPropertyName)
            local requiredMarker = getRequiredMarker(required, "@NonNull ")

            local code = string.format("    private %sNumber %s;\n", requiredMarker,
                currentPropertyName);

            global_context:addProperties("visitNumberProperty", parentModelName,
                { WriteOperation.new_append(code, parentModelName) })
        else
            error("Unknown parent type for Number")
        end
    end
    return {}
end

local function beforeDecorator()
    global_context:addLastChildrenModelName("visitNumberProperty", "Number")
end

return functionCallAndLog("visitNumberProperty", visitNumberProperty, beforeDecorator)
