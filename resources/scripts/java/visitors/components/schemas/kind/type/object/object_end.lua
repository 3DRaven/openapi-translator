local function getCollectedCode(currentModelName)
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
end

--- This visitor is invoked after all the content inside a schema of type object has been processed.
--- Returns a code of the end of an object based on whether it's required.
--- @param objectDescriptor ObjectType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitObjectEnd(objectDescriptor, extensions, callsStack)
    local currentModelName = getCurrentModelNameMandatory(namesStack)

    local parentModelName = getParentModelName(namesStack)
    -- if this object has parent, it must save something to parent about it self
    if parentModelName ~= nil then
        -- this object must save it model name for parent if it exists
        global_context:addLastChildrenModelName("visitObjectEnd", getCurrentModelNameMandatory(namesStack))
        -- For parent OBJECT we need to write property to it
        if hasSpecifiedParentsInCallChain("visitObjectEnd",
                callsStack,
                { Script.OBJECT_PROPERTY_START }) then
            local currentPropertyName = getCurrentPropertyNameMandatory(namesStack)
            generateSimplePropertyCode("visitObjectEnd",
                parentModelName,
                currentPropertyName,
                currentModelName,
                "@Nonnull",
                "import javax.annotation.Nonnull;\n\n"
            )

            return getCollectedCode(currentModelName)
        elseif hasSpecifiedParentsInCallChain("visitObjectEnd", callsStack, { Script.ALL_OF_START }) then
            --- If the parent is allOf, we need to place all created properties and other of this object into the parent.
            local collectedProperties = global_context:getProperties("visitObjectEnd", currentModelName)
            global_context:adaptProperties("visitObjectEnd", parentModelName, collectedProperties)
            local collectedIncludes = global_context:getIncludes("visitObjectEnd", currentModelName)
            global_context:adaptIncludes("visitObjectEnd", parentModelName, collectedIncludes)
            local collectedMethods = global_context:getMethods("visitObjectEnd", currentModelName)
            global_context:adaptMethods("visitObjectEnd", parentModelName, collectedMethods)
        end
    else
        return getCollectedCode(currentModelName)
    end

    return {}
end

local function beforeDecorator(namesStack)
end


return functionCallAndLog("visitObjectEnd", visitObjectEnd, beforeDecorator)
