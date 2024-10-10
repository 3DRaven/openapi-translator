--- This visitor is invoked at externalDocs of spec
--- @param externalDocsDescriptor ExternalDocsDescriptor # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table<string,any> # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecExternalDocs(externalDocsDescriptor, extensions)
    return {}
end

return functionCallAndLog("visitSpecExternalDocs", visitSpecExternalDocs)
