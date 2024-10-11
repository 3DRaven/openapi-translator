--- This visitor is invoked after processing response header
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param description string | nil # A brief description of the parameter. May include CommonMark syntax for rich text representation.
--- @param style string        # The style of the header.
--- @param required boolean         # Indicates if the parameter is mandatory. Must be true if located in "path".
--- @param deprecated boolean | nil # Specifies if the parameter is deprecated and should be phased out.
--- @param format table # The format of the parameter schema or content.
--- @param example table | nil        # An example value of the parameter.
--- @param examples table<string, table> # A map of examples associated with the parameter.
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitResponseHeaderEnd(namesStack, description, style, required, deprecated, format, example, examples,
                                extensions)
    return {}
end

return functionCallAndLog("visitResponseHeaderEnd", visitResponseHeaderEnd)
