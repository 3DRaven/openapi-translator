--- This visitor is invoked before processing response if reference
--- @param responseName string|null #
--- @param responseReference string #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitResponseReference(responseName, responseReference, extensions, callId)
    return {}
end

return functionCallAndLog("visitResponseReference", visitResponseReference)
