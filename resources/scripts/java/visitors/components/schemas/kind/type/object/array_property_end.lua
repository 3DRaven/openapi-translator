--- This visitor is invoked to complete processing a schema containing an array, regardless of whether
--- the schema is nested within an object or is a top-level schema. The invocation occurs after processing
--- the schemas contained within the array.
--- Returns a code for array model end
--- @param arrayDescriptor ArrayType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitArrayPropertyEnd(arrayDescriptor, extensions, callsStack)
    local parentModelName = getParentModelName(namesStack)
    local currentModelName = getCurrentModelNameMandatory(namesStack)
    local currentPropertyName = getCurrentPropertyNameMandatory(namesStack)
    local lastChildrenModelName = global_context:getLastChildrenModelName("visitArrayPropertyEnd")

    if lastChildrenModelName == nil then
        error("Unknown model for items")
    else
        -- if it is root object as array we must generate full model
        if parentModelName == nil then
            local parameters = { className = currentModelName, childClassName = lastChildrenModelName }

            local code = interpolate(parameters, formatAndTrimIndent([[
            import java.util.List;

            public class ${className} {
                private List<${childClassName}> items;
                public ${className}() {}
                public ${className}(List<${childClassName}> items) {
                    this.items = items;
                }
                public List<${childClassName}> get${className}() {
                    return items;
                }
                public void set${className}(List<${childClassName}> items) {
                    this.items = items;
                }
            }
            ]]))
            -- last children used and it can be forgotten
            global_context:dropLastChildrenModelName("visitArrayPropertyEnd")
            return { WriteOperation.new_append(code, currentModelName) }
        else -- if it is just property for object or additionalProperties we need to write some to parents
            if hasSpecifiedParentsInCallChain("visitArrayPropertyEnd",
                    callsStack, { Script.OBJECT_START }) then
                -- Adding the import at the beginning of the parent model file
                global_context:addIncludes("visitArrayPropertyEnd", parentModelName,
                    { WriteOperation.new_append("import java.util.List;\n\n", parentModelName) })

                local code = string.format("    private List<%s> %s = new List<>();\n",
                    lastChildrenModelName, currentPropertyName);

                global_context:addProperties("visitArrayPropertyEnd", parentModelName,
                    { WriteOperation.new_append(code, parentModelName) })

                -- last children didn't droped because on visitObjectPropertyEnd it will be dropped
                -- global_context:dropLastChildrenModelName("visitArrayPropertyEnd")
            elseif hasSpecifiedParentsInCallChain("visitArrayPropertyEnd",
                    callsStack, { Script.ARRAY_PROPERTY_START, Script.OBJECT_ADDITIONAL_PROPERTIES_START }) then
                -- additionalProperties with array with List<lastChildrenModelName>
                local code = string.format("List<%s>", lastChildrenModelName);
                -- last children used and it can be forgotten
                global_context:dropLastChildrenModelName("visitArrayPropertyEnd")
                -- now for parent we child with model List<lastChildrenModelName>
                global_context:addLastChildrenModelName("visitArrayPropertyEnd", code)
            end
            return {}
        end
    end
end

local function beforeDecorator()
end

return functionCallAndLog("visitArrayPropertyEnd", visitArrayPropertyEnd, beforeDecorator)
