--- This visitor is invoked for processing example in response header examples
--- @param namesStack ModelName[]       # chain of model names from root to this point
--- @param summary string|nil           # Short description for the example.
--- @param description string|nil       # Long description for the example; may use CommonMark syntax.
--- @param value table|nil              # Embedded literal example; mutually exclusive with external_value.
--- @param external_value string|nil    # URL pointing to the example; mutually exclusive with value.
--- @param extensions table             # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[]            # Returns the output code and  file name for writing code
function visitExamplesExample(namesStack, summary, description, value, external_value, extensions,
                              callsStack)
    return {}
end

return functionCallAndLog("visitExamplesExample", visitExamplesExample)
