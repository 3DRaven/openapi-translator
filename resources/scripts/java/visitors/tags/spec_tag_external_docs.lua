--- This visitor is invoked at externalDocs of tag
--- @param externalDocsDescriptor ExternalDocsDescriptor # table with free form with "x-" OpenAPI extensions for tag
--- @param extensions table<string,any> # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecTagExternalDocs(externalDocsDescriptor, extensions)
    return {}
end

return functionCallAndLog("visitSpecTagExternalDocs", visitSpecTagExternalDocs)
