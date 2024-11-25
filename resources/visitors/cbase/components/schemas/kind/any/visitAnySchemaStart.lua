--- This visitor is invoked to process a found schema without a defined structure
--- such as `additionalProperties: {}`.
--- Returns a model name with unknown structure (it is {} in OpenAPI)
--- @param anySchemaDescriptor AnySchema # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitAnySchemaStart(anySchemaDescriptor, extensions, callId)
    --- @type ModelBase?
    local currentModel = GLOBAL_CONTEXT.models:peek()
    -- it is additionalProperties without structure (additionalProperties: {})
    -- or array with object items type
    if currentModel ~= nil and currentModel:instanceOf(TypeTransferModel) then
        local codeVariant = CODE.getVariant(extensions[Extensions.VARIANT])
        currentModel.name = codeVariant:getAnyType()
    else
        -- It is unknown combination of properties (redundant or not allowed by OpenAPI 3)
        -- so, we process this as special type of model
        -- translator try to convert any schema to all of grouped types as allOf, anyOf and other
        -- and then try to convert to concrete types as string and other if type set in schema
        -- so, after calling visitAnySchemaStart may be called visitors for all possible variants
        -- if they exists
        print(formatAndTrimIndent([[
            Found unknown combination of properties in OpenAPI 3 spec,
            will be used separated sets of names for every known parts
        ]]))
        GLOBAL_CONTEXT.models:push(AnySchemaModel.new("any-schema"))
    end
    return {}
end

return functionCallAndLog("visitAnySchemaStart", visitAnySchemaStart)
