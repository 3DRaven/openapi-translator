--- @class NumberDescriptor
--- @field format string|nil Describes the format of the number.
--- @field multiple_of number|nil An optional field specifying that the value must be a multiple of this number.
--- @field exclusive_minimum boolean An optional field indicating if the minimum value is exclusive.
--- @field exclusive_maximum boolean An optional field indicating if the maximum value is exclusive.
--- @field minimum number|nil The minimum allowed value for this number.
--- @field maximum number|nil The maximum allowed value for this number.
--- @field enumeration number[]|nil An optional list of allowed values for this number.

--- This visitor is invoked when a property of type number is found.
--- Returns a code for creating property for number value
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param numberDescriptor StringDescriptor # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitNumberProperty(namesStack, numberDescriptor, extensions)
    local parentModelName = getParentModelName(namesStack)
    if parentModelName == nil then
        --- This is possible if a schema is constructed as a separate value;
        --- such a schema is not a model but can be used as a reference in other schemas.
        --- For example, you can add descriptions and other elements to it instead of copying them
        --- within the specification. So, we simply don't need to do anything in this case.
        print("Number property without parent skipt")
    else
        local parentType = global_context:getLastParentType("visitNumberProperty")
        if parentType == ParentType.OBJECT or
            parentType == ParentType.ALL_OF then
            local currentPropertyName = getCurrentPropertyNameMandatory(namesStack)
            local required = global_context:isPropertyRequired("visitNumberProperty", parentModelName,
                currentPropertyName)
            local requiredMarker = getRequiredMarker(required, "@NonNull ")

            local code = string.format("    private %sNumber %s;\n", requiredMarker,
                currentPropertyName);

            global_context:addProperties("visitNumberProperty", parentModelName,
                { WriteOperation.new_append(code, parentModelName) })
        elseif parentType == ParentType.ARRAY then
        elseif parentType == ParentType.ADDITIONAL then
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
