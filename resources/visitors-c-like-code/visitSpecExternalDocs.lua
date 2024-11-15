--- This visitor is invoked at externalDocs of spec
--- @param externalDocsDescriptor ExternalDocumentation #
--- @param extensions table<string,any> # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSpecExternalDocs(externalDocsDescriptor, extensions, callId)
    return {}
end

return functionCallAndLog("visitSpecExternalDocs", visitSpecExternalDocs)
