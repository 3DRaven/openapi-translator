local module = {}

--- Returns a Property based on descriptor and extensions
--- @param currentModel ObjectModel # model that contains required markers for property
--- @param propertyName string # name of the property
--- @param descriptor table # property descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return Property # Returns the Property
function module.createProperty(currentModel, propertyName, descriptor, extensions)
    local codeVariant = CODE.getVariant(extensions[Extensions.VARIANT])
    local property = Property.new(propertyName, codeVariant:getStringType(), descriptor, extensions)

    if currentModel ~= nil then
        if currentModel:isPropertyRequired(property.name) then
            property:addInclude(codeVariant:getRequiredImport())
            property:addMarker(codeVariant:getRequiredMarker())
        end
    end

    property:addMarker(codeVariant:getCustomMarkers())
    property:addInclude(codeVariant:getCustomImports())

    return property
end

return module
