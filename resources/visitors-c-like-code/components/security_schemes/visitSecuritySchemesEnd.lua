--- This visitor is invoked after processing security schemes
--- @param securitySchemes table<string,ReferenceOr<table>> # generic collection of security schemes
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some usefull identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitSecuritySchemesEnd(securitySchemes, extensions, callId)
    return {}
end

return functionCallAndLog("visitSecuritySchemesEnd", visitSecuritySchemesEnd)
