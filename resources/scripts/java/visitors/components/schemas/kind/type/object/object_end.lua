--- This visitor is invoked after all the content inside a schema of type object has been processed.
--- Returns a code of the end of an object based on whether it's required.
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param parentType ParentType # Type of parent for this property
--- @param objectDescriptor ObjectDescriptor # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitObjectEnd(namesStack, parentType, objectDescriptor, extensions)
    local currentModelName = getCurrentModelNameMandatory(namesStack)

    local parentModelName = getParentModelName(namesStack)
    -- if this object has parent, it must save something to parent about it self
    if parentModelName ~= nil then
        -- this object must save it model name for parent if it exists
        global_context:addLastChildrenModelName("visitObjectEnd", getCurrentModelNameMandatory(namesStack))
        -- it parent for this object
        local parentType = global_context:getLastParentType("visitObjectEnd")
        -- For parent OBJECT we need to write property to it
        if parentType == ParentType.OBJECT then
            local currentPropertyName = getCurrentPropertyNameMandatory(namesStack)
            local required = global_context:isPropertyRequired("visitObjectEnd", parentModelName,
                currentPropertyName)
            local requiredMarker = getRequiredMarker(required, "@NonNull ")

            local code = string.format("    private %s%s %s;\n", requiredMarker, currentModelName,
                currentPropertyName);

            global_context:addProperties("visitObjectEnd", parentModelName,
                { WriteOperation.new_append(code, parentModelName) })

            -- this object must to save self model
            local model = global_context:getModel("visitObjectEnd", currentModelName)
            if model == nil then
                print("Model [" ..
                    currentModelName .. "] not found in global_context by [visitObjectEnd], generated empty object")
                return { WriteOperation.new_append(string.format("public class %s {\n\n}\n", currentModelName),
                    currentModelName) }
            else
                return concatTables(
                    model.includes,
                    { WriteOperation.new_append(string.format("public class %s {\n\n", currentModelName),
                        currentModelName) },
                    model.properties,
                    model.methods,
                    { WriteOperation.new_append("\n}\n", currentModelName) })
            end
        elseif parentType == ParentType.ARRAY or parentType == ParentType.ADDITIONAL then --we already saved model name of this object as lastChildrenModel name
        elseif parentType == ParentType.ALL_OF then
            --- If the parent is allOf, we need to place all created properties and other of this object into the parent.
            local collectedProperties = global_context:getProperties("visitObjectEnd", currentModelName)
            global_context:adaptProperties("visitObjectEnd", parentModelName, collectedProperties)
            local collectedIncludes = global_context:getIncludes("visitObjectEnd", currentModelName)
            global_context:adaptIncludes("visitObjectEnd", parentModelName, collectedIncludes)
            local collectedMethods = global_context:getMethods("visitObjectEnd", currentModelName)
            global_context:adaptMethods("visitObjectEnd", parentModelName, collectedMethods)
        else
            error("Unknown parent type for object")
        end
    end

    return {}
end

local function beforeDecorator(namesStack)
    -- drop before main code because we need to know parent for this object if it exists, this object not a parent now
    global_context:dropLastParentType("visitObjectEnd")
end


return functionCallAndLog("visitObjectEnd", visitObjectEnd, beforeDecorator)
