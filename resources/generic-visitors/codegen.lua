local module = {}

--- @param currentModel ModelBase
--- @param type string
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[]
function module.addAdditionalProperty(currentModel, type, extensions)
    -- Adding the import at the beginning of the current model file
    currentModel:adaptToIncludes({ WriteOperation.new_prepend(PARTS.getAdditionalPropertiesImport(),
        currentModel.name) })
    local propertyName = getFirstExistsName(extensions[Extensions.ADDITIONAL_PROPERTY_NAME], "additionalProperties")
    local code = PARTS.getAdditionalPropertiesProperty(type, propertyName);

    currentModel:addModelProperty(propertyName, extensions)
    currentModel:adaptToLastProperty({ WriteOperation.new_append(code, currentModel.name) })
    return {}
end

--- @param currentModel ModelBase
--- @param type string
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[]
function module.addGenericPropertyCode(currentModel, type, extensions)
    if currentModel == nil then
        --- This is possible if a schema is constructed as a separate value;
        --- such a schema is not a model but can be used as a reference in other schemas.
        --- For example, you can add descriptions and other elements to it instead of copying them
        --- within the specification. So, we simply don't need to do anything in this case.
        print(type .. " property without parent skipt")
    else
        if currentModel:instanceOf(ObjectModel) or currentModel:instanceOf(AllOfModel) then
            ---@type Property?
            local property = currentModel.properties:peek()
            if property ~= nil then
                local codeExtension = extensions[Extensions.CODE_BEFORE]
                local codeBefore = nil

                if codeExtension ~= nil then
                    for _, it in ipairs(codeExtension) do
                        local import = it[Extensions.IMPORT]
                        if import ~= nil then
                            currentModel.includes:push(WriteOperation.new_append(import .. "\n", currentModel.name))
                        end
                        local code = it[Extensions.CODE]
                        if codeBefore == nil and code then
                            codeBefore = "    " .. code .. "\n"
                        elseif codeBefore ~= nil and code then
                            codeBefore = codeBefore .. "    " .. code .. "\n"
                        end
                    end
                end

                local requiredMarker

                if currentModel:isPropertyRequired(property.name) then
                    currentModel.includes:push(WriteOperation.new_prepend(PARTS.getRequiredImport(), currentModel.name))
                    requiredMarker = PARTS.getRequiredMarker()
                end

                local code = PARTS.getPropertyCode(codeBefore or "", requiredMarker or "", type, property.name);

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
