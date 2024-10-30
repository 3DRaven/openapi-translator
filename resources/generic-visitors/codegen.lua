local module = {}

--- @param model ModelBase
--- @param type string
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function module.addGenericPropertyCode(model, type, extensions)
    if model == nil then
        --- This is possible if a schema is constructed as a separate value;
        --- such a schema is not a model but can be used as a reference in other schemas.
        --- For example, you can add descriptions and other elements to it instead of copying them
        --- within the specification. So, we simply don't need to do anything in this case.
        print(type .. " property without parent skipt")
    else
        if model:instanceOf(ObjectModel) or model:instanceOf(AllOfModel) then
            local codeExtension = extensions[Extensions.CODE_BEFORE]
            local codeBefore = nil

            if codeExtension ~= nil then
                for _, it in ipairs(codeExtension) do
                    local import = it[Extensions.IMPORT]
                    if import ~= nil then
                        model.includes:push(WriteOperation.new_append(import .. "\n", model.name))
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

            ---@type Property
            local property = model.properties:element()
            if model:isPropertyRequired(property.name) then
                model.includes:push(WriteOperation.new_prepend(getRequiredImport(), model.name))
                requiredMarker = getRequiredMarker()
            end

            local code = getPropertyCode(codeBefore or "", requiredMarker or "", type, property.name);

            property.code:push(WriteOperation.new_append(code, model.name))
        elseif model:instanceOf(TypeTransferModel) then
            -- This is necessary for additionalProperties and type: array since they independently generate code for themselves
            model.name = type
        end
    end
    return {}
end

return module
