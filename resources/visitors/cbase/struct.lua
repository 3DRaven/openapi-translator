local module = {}

--- @param currentModel ModelBase
--- @param type string
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[]
function module.addAdditionalProperty(currentModel, type, extensions)
    local codeVariant = CODE.getVariant(extensions[Extensions.VARIANT])
    -- Adding the import at the beginning of the current model file
    currentModel:adaptToIncludes({ WriteOperation.new_prepend(codeVariant:getAdditionalPropertiesImport(),
        currentModel.name) })
    --- @type string
    --- @diagnostic disable-next-line: assign-type-mismatch
    local propertyName = extensions[Extensions.ADDITIONAL_PROPERTY_NAME] or "additionalProperties"
    local code = codeVariant:getAdditionalPropertiesProperty(type, propertyName);

    currentModel:addModelProperty(propertyName, extensions)
    currentModel:adaptToLastProperty({ WriteOperation.new_append(code, currentModel.name) })
    return {}
end

--- @param currentModel ModelBase?
--- @param type string
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[]
function module.addGenericPropertyCode(currentModel, type, extensions)
    if currentModel == nil then
        --- This is possible if a schema is constructed as a separate value;
        --- such a schema is not a model but can be used as a reference in other schemas.
        --- For example, you can add descriptions and other elements to it instead of copying them
        --- within the specification. So, we simply don't need to do anything in this case.
        print(type .. " property without parent skip")
    else
        if currentModel:instanceOf(ObjectModel) or currentModel:instanceOf(AllOfModel) then
            ---@type Property?
            local property = currentModel.properties:peek()
            if property ~= nil then
                local requiredMarker
                local codeVariant = CODE.getVariant(extensions[Extensions.VARIANT])
                if currentModel:isPropertyRequired(property.name) then
                    currentModel:adaptToIncludes({ WriteOperation.new_prepend(codeVariant:getRequiredImport(),
                        currentModel.name) })
                    requiredMarker = codeVariant:getRequiredMarker()
                end

                local customMarkers = codeVariant:getCustomMarkers()
                if customMarkers ~= nil then
                    local customImports = codeVariant:getCustomImports()
                    if customImports ~= nil then
                        currentModel:adaptToIncludes({ WriteOperation.new_prepend(customImports,
                            currentModel.name) })
                    end
                end

                local code = codeVariant:getPropertyCode(customMarkers, requiredMarker, type, property.name);

                property.code:push(WriteOperation.new_append(code, currentModel.name))
            else
                print("No properties found")
            end
        elseif currentModel:instanceOf(TypeTransferModel) then
            -- This is necessary for additionalProperties and type: array since they independently generate code for themselves
            currentModel.name = type
        end
    end
    return {}
end

return module
