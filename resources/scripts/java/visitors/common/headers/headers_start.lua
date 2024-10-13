--- Represents a reference or an item.
---@class ReferenceOr<T>
---@field reference string # The reference string, applicable if it is a reference.
---@field item any

--- Represents the style of a header parameter.
---@class HeaderStyle
---@field Simple string # Simple style parameters defined by RFC6570.

--- Defines the schema or content representation for a parameter.
---@class ParameterSchemaOrContent
---@field Schema ReferenceOr|nil # The schema defining the type used for the parameter.
---@field Content Content        # A map containing media type representations for the parameter. Must contain only one entry.

--- Represents a map from media type to its definition.
---@class Content : table<string, MediaType>

--- Describes a media type with potentially multiple examples and encoding information.
---@class MediaType
---@field schema ReferenceOr|nil # The schema defining the content of the request, response, or parameter.
---@field example any|nil        # An example of the media type in the specified format; mutually exclusive with examples.
---@field examples table<string, ReferenceOr>|nil # Examples matching the media type and schema; mutually exclusive with example.
---@field encoding table<string, Encoding>|nil      # Map between a property name and its encoding information; applies to specific media types.
---@field extensions table<string, any>          # Inline extensions to this object.

--- Represents an example of a media type.
---@class Example
---@field summary string|nil             # Short description for the example.
---@field description string|nil         # Long description for the example; may use CommonMark syntax.
---@field value any|nil                  # Embedded literal example; mutually exclusive with external_value.
---@field external_value string|nil      # URL pointing to the example; mutually exclusive with value.
---@field extensions table<string, any>  # Inline extensions to this object.

--- Describes encoding properties for a specific field in a request body.
---@class Encoding
---@field content_type string|nil                # Content-Type for encoding a specific property.
---@field headers table<string, ReferenceOr>|nil # Additional headers for multipart media type (excluding Content-Type).
---@field style QueryStyle|nil                   # Serialization style for a specific property.
---@field explode boolean|nil                    # Determine separate parameters for array/object values; default to false.
---@field allow_reserved boolean|nil             # Whether reserved characters are allowed without percent-encoding; default to false.
---@field extensions table<string, any>          # Inline extensions to this object.

--- Defines how parameters are serialized for query strings or form parameters.
---@class QueryStyle
---@field Form string          # Form style parameters defined by RFC6570.
---@field SpaceDelimited string # Parameters separated by spaces.
---@field PipeDelimited string  # Parameters separated by pipes.
---@field DeepObject string     # Nested objects rendered using form parameters.

--- Represents the headers parameter, which is a map from strings to references or items.
---@class Header
---@field description string | nil # A brief description of the parameter. May include CommonMark syntax for rich text representation.
---@field style HeaderStyle        # The style of the header.
---@field required boolean         # Indicates if the parameter is mandatory. Must be true if located in "path".
---@field deprecated boolean | nil # Specifies if the parameter is deprecated and should be phased out.
---@field format ParameterSchemaOrContent # The format of the parameter schema or content.
---@field example table | nil        # An example value of the parameter.
---@field examples table<string, ReferenceOr> # A map of examples associated with the parameter.
---@field extensions table<string, any>       # Inline extensions to this object.

--- This visitor is invoked before processing all response headers
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param headers table<string, Header> # Represents the headers parameter, which is a map from strings to references or items.
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitHeadersStart(namesStack, headers, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitHeadersStart", visitHeadersStart)
