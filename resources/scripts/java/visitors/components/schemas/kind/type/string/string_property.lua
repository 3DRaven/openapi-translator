--- Represents property of type string.
---@class StringDescriptor
---@field format string | nil # The format of the string type
---@field pattern string | nil                 # The pattern for the string type
---@field enum string[] | nil                  # The enumeration of possible string values
---@field min_length integer | nil              # The minimum length of the string
---@field max_length integer | nil              # The maximum length of the string

--- This visitor is invoked when a property of type string is found.
--- Returns a string based on the provided string descriptor.
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param stringDescriptor StringDescriptor # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitStringProperty(namesStack, stringDescriptor, extensions)
    local parentModelName = getParentModelName(namesStack)
    if parentModelName == nil then
        --- This is possible if a schema is constructed as a separate value;
        --- such a schema is not a model but can be used as a reference in other schemas.
        --- For example, you can add descriptions and other elements to it instead of copying them
        --- within the specification. So, we simply don't need to do anything in this case.
        print("String property without parent skipt")
    else
        local parentType = global_context:getLastParentType("visitStringProperty")
        if parentType == ParentType.OBJECT or
            parentType == ParentType.ALL_OF then
            local currentPropertyName = getCurrentPropertyNameMandatory(namesStack)
            local required = global_context:isPropertyRequired("visitStringProperty", parentModelName,
                currentPropertyName)
            local requiredMarker = getRequiredMarker(required, "@NonNull ")

            local code = string.format("    private %sString %s;\n", requiredMarker,
                currentPropertyName);

            global_context:addProperties("visitStringProperty", parentModelName,
                { WriteOperation.new_append(code, parentModelName) })
        elseif parentType == ParentType.ARRAY then
        elseif parentType == ParentType.ADDITIONAL then
        else
            error("Unknown parent type for String")
        end
    end
    return {}
end

local function beforeDecorator()
    global_context:addLastChildrenModelName("visitStringProperty", "String")
end

return functionCallAndLog("visitStringProperty", visitStringProperty, beforeDecorator)
