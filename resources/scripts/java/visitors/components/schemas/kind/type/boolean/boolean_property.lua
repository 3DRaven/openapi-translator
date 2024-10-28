--- This visitor is invoked when a property of type boolean is found.
--- Returns a code for creating storage for additionalProperties (Map as example)
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param booleanDescriptor BooleanType # boolean property descriptor
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitBooleanProperty(booleanDescriptor, extensions, callsStack)
    local parentModelName = getParentModelName(namesStack)
    if parentModelName == nil then
        --- This is possible if a schema is constructed as a separate value;
        --- such a schema is not a model but can be used as a reference in other schemas.
        --- For example, you can add descriptions and other elements to it instead of copying them
        --- within the specification. So, we simply don't need to do anything in this case.
        print("Boolean property without parent skipt")
    else
        if hasSpecifiedParentsInCallChain("visitBooleanProperty",
                callsStack,
                { Script.OBJECT_PROPERTY_START, Script.ALL_OF_START }) then
            local currentPropertyName = getCurrentPropertyNameMandatory(namesStack)
            generateSimplePropertyCode("visitObjectEnd",
                parentModelName,
                currentPropertyName,
                "Boolean",
                "@Nonnull",
                "import javax.annotation.Nonnull;\n\n"
            )
        end
    end
    return {}
end

local function beforeDecorator()
    global_context:addLastChildrenModelName("visitBooleanProperty", "Boolean")
end

return functionCallAndLog("visitBooleanProperty", visitBooleanProperty, beforeDecorator)
