--- This visitor is invoked for processing example in response header examples
--- @param namesStack ModelName[]       # chain of model names from root to this point
--- @param summary string|nil           # Short description for the example.
--- @param description string|nil       # Long description for the example; may use CommonMark syntax.
--- @param value table|nil              # Embedded literal example; mutually exclusive with external_value.
--- @param external_value string|nil    # URL pointing to the example; mutually exclusive with value.
--- @param extensions table             # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[]            # Returns the output code and  file name for writing code
function visitResponseHeaderExamplesExample(namesStack, summary, description, value, external_value, extensions)
    return {}
end

return functionCallAndLog("visitResponseHeaderExamplesExample", visitResponseHeaderExamplesExample)
