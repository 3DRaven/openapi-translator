--- This visitor is invoked after processing response if reference
--- @param responseName string|null #
--- @param responseReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitResponseReferenceEnd(responseName, responseReference, extensions, callId)
    -- if we found reference we every time use names from reference or from extension
    GLOBAL_CONTEXT.names:clear()
    GLOBAL_CONTEXT.names:pushAll(GLOBAL_CONTEXT.savedNames.items)
    GLOBAL_CONTEXT.savedNames:clear()
    return {}
end

return functionCallAndLog("visitResponseReferenceEnd", visitResponseReferenceEnd, -1)
