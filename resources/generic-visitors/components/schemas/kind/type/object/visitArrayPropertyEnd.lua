--- This visitor is invoked to complete processing a schema containing an array, regardless of whether
--- the schema is nested within an object or is a top-level schema. The invocation occurs after processing
--- the schemas contained within the array.
--- Returns a code for array model end
--- @param arrayDescriptor ArrayType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitArrayPropertyEnd(arrayDescriptor, extensions, callsStack)
    --- @type ModelBase
    local childModel = global_context.models:pop()
    --- @type ModelBase?
    local currentModel = global_context.models:peek()

    if childModel.name == nil then
        error("Unknown model for items")
    else
        -- if it is root object as array we must generate full model
        if currentModel == nil then
            local arrayModelName = global_context.names:element()
            local parameters = { className = arrayModelName, childClassName = childModel.name }

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
            return { WriteOperation.new_append(code, arrayModelName) }
        else -- if it is just property for object or additionalProperties we need to write some to parents
            if currentModel:instanceOf(ObjectModel) then
                --- @type Property
                local property = currentModel.properties:element()
                -- Adding the import at the beginning of the parent model file
                currentModel:adaptToIncludes({ WriteOperation.new_append("import java.util.List;\n", currentModel.name) })
                local code = string.format("    private List<%s> %s = new List<>();\n",
                    childModel.name, property.name);
                currentModel:adaptToLastProperty({ WriteOperation.new_append(code, currentModel.name) })
            elseif currentModel:instanceOf(TypeTransferModel) then
                -- additionalProperties with array with List<lastChildrenModelName>
                -- now for parent we child with model List<lastChildrenModelName>
                currentModel.name = "List<" .. childModel.name .. ">"
            end
            return {}
        end
    end
end

local function beforeDecorator()
end

return functionCallAndLog("visitArrayPropertyEnd", visitArrayPropertyEnd, beforeDecorator)