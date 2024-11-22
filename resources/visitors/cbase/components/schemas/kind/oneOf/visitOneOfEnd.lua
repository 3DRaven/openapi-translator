--- This visitor is invoked before processing the found schema in oneOf element
--- @param schemas ReferenceOr<Schema>[] #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitOneOfEnd(schemas, extensions, callId)
    ---@type OneOfModel
    local currentModel = GLOBAL_CONTEXT.models:pop()

    if currentModel == nil then
        error("Model for oneOf not found")
    else
        -- all models already saved, so we do nothing
        return {}
    end
end

return functionCallAndLog("visitOneOfEnd", visitOneOfEnd)
