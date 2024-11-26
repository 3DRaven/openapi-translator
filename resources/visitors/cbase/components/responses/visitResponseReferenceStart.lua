--- This visitor is invoked before processing response if reference
--- @param responseName string|null #
--- @param responseReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitResponseReferenceStart(responseName, responseReference, extensions, callId)
    -- if we found reference we every time use names from reference or from extension
    GLOBAL_CONTEXT.savedNames:pushAll(GLOBAL_CONTEXT.names.items)
    GLOBAL_CONTEXT.names:clear()
    GLOBAL_CONTEXT.names:push(extensions[Extensions.MODEL_NAME] or lastReferencePart(responseReference))
    return {}
end

return functionCallAndLog("visitResponseReferenceStart", visitResponseReferenceStart)
