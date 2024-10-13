--- Represents a catch-all for any combination of properties that doesn't correspond to one of the
--- predefined subsets.
---@class AnySchema
---@field typ string | nil                      # The type of the schema
---@field pattern string | nil                  # The pattern in the schema
---@field multiple_of number | nil              # A multiple constraint for numeric types
---@field exclusive_minimum boolean | nil       # Indicates if there is an exclusive minimum constraint
---@field exclusive_maximum boolean | nil       # Indicates if there is an exclusive maximum constraint
---@field minimum number | nil                  # The minimum value for numeric types
---@field maximum number | nil                  # The maximum value for numeric types
---@field properties table<string, table> | nil  # Properties defined in the schema
---@field required string[] | nil               # Required properties in the schema
---@field additional_properties table | nil   # Additional properties definition
---@field min_properties integer | nil           # Minimum number of properties allowed
---@field max_properties integer | nil           # Maximum number of properties allowed
---@field items table | nil       # Items definition for array types
---@field min_items integer | nil                # Minimum number of items in an array
---@field max_items integer | nil                # Maximum number of items in an array
---@field unique_items boolean | nil            # Indicates if array items must be unique
---@field enumeration table[] | nil             # Enumeration of possible values
---@field format string | nil                   # Format of the schema
---@field min_length integer | nil               # Minimum length for string types
---@field max_length integer | nil               # Maximum length for string types
---@field one_of table | nil                    # Array of schemas where at least one should match
---@field all_of table | nil                    # Array of schemas where all should match
---@field any_of table | nil                    # Array of schemas where any can match
---@field not table | nil                       # Schema that must not match

--- This visitor is invoked to process a found schema without a defined structure,
--- such as `additionalProperties: {}`.
--- Returns a model name with unknown structure (it is {} in OpenAPI)
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param required boolean # Indicates if the property value (this object) is required
--- @param anySchemaDescriptor AnySchema # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitAnySchema(namesStack, required, anySchemaDescriptor, extensions, callsStack)
    return {}
end

local function beforeDecorator()
    global_context:addLastChildrenModelName("visitAnySchema", "Object")
end

return functionCallAndLog("visitAnySchema", visitAnySchema, beforeDecorator)
