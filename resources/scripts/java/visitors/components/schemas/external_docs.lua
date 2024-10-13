--- This visitor is invoked at externalDocs of schema
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param externalDocsDescriptor ExternalDocsDescriptor # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSpecExternalDocs(namesStack, externalDocsDescriptor, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSpecExternalDocs", visitSpecExternalDocs)
