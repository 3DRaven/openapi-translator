--- This visitor is invoked at externalDocs of tag
--- @param externalDocs ExternalDocumentation # A short description of the target documentation
--- @param extensions table<string,any> # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitExternalDocs(externalDocs, extensions, callId)
    return {}
end

return functionCallAndLog("visitExternalDocs", visitExternalDocs)
