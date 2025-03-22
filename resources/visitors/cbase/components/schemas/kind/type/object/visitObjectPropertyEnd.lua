local function inspectIndexChain(mt)
    local depth = 0
    while mt and mt.__index do
        print("Inspecting __index at level " .. depth .. ":")
        for key, value in pairs(mt.__index) do
            print("  Field:", key, "Value:", value)
        end
        mt = getmetatable(mt.__index)
        depth = depth + 1
    end
end

--- This visitor handles the processing of object schema property.
--- @param propertyName string|null #
--- @param schema Schema # free form of additionalProperties has this value
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitObjectPropertyEnd(propertyName, schema, extensions, callId)
    local value = GLOBAL_CONTEXT.values:pop()
    if value:instanceOf(ObjectModel) then
        --- @type ObjectModel
        value = value
        local codeVariant = CODE.getVariant(extensions[Extensions.VARIANT])
        local currentModel = GLOBAL_CONTEXT.values:elementObjectModel()
        --- because we need remove current property name from names stack we didn't use propertyName
        local stackPropertyName = GLOBAL_CONTEXT.names:pop()
        local classFileName = codeVariant:getClassFileName(concatStackCleanCapitalized(GLOBAL_CONTEXT.names))
        local objectCode = value:getCode(extensions)
        value = STRUCT.createProperty(currentModel, stackPropertyName, schema, extensions)
        GLOBAL_CONTEXT.values:elementObjectModel():addProperty(value)
        return { WriteOperation.new_from_code(objectCode, classFileName) }
    else
        GLOBAL_CONTEXT.values:elementObjectModel():addProperty(value)
        return {}
    end
end

return functionCallAndLog("visitObjectPropertyEnd", visitObjectPropertyEnd, -1)
