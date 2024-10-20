--- This visitor is invoked for processing OAuth2 password flow
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param flow PasswordOAuth2Flow #
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitSecuritySchemeOAuth2FlowPassword(namesStack, flow, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitSecuritySchemeOAuth2FlowPassword", visitSecuritySchemeOAuth2FlowPassword)
