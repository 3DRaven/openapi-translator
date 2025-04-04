--- This section contains all global functions and variables that are created before the visitors
--- start working.

local DEFAULT_PRINT_ARGS_INDENT = 8

local CALLS = {}
local INDENT = 0

-----------------------------------------------------------------------------------------------------------------
--- Constructions below is used solely to inform the Lua language server
--- about the existence of the global variable for convenience when writing scripts.
--- variable already set by Rust code

--- Global variable containing parameters passed by the translator to the Lua code either from the OpenAPI
--- specification (x-ot-target-parameters extension) or from command line parameters.
--- Command line parameters take precedence over API specification parameters.
if false then
    ---@type any|null|nil # The type depends on how the parameters are specified in the command line or OpenAPI specification
    TARGET_PARAMETERS = nil
end

if false then
    ---@type null # it is predefined in rust code value with type null
    NULL = nil
end

if false then
    ---@type string # path to visitors scripts
    VISITORS_PATH = nil
end

if false then
    ---@type string # path to target scripts
    TARGET_PATH = nil
end
---------------------------------------------------------------------------------------------------------

--- TYPES -----------------------------------------------------------------------------------------------

--- It is a special predefined global value similar to nil. However, it
--- specifically used for data passed from the translator (Rust code) that has a nil value.
--- userdata(nil) == NULL
--- @class null

--- The description of incoming types is automatically generated based on the Rust structures.
--- They represent an approximate content of the structures; for example, since Lua does not have Enums,
--- a string will be used in place of fields.

--- Represents a set of reusable components for different aspects of the OpenAPI Specification (OAS).
--- All objects defined within the components will not impact the API unless they are explicitly referenced
--- from properties outside the components.
--- @class Components
--- A table to hold reusable Schema Objects.
--- @field schemas table<string, ReferenceOr> A table mapping strings to Schema reference objects.
--- A table to hold reusable Response Objects.
--- @field responses table<string, ReferenceOr> A table mapping strings to Response reference objects.
--- A table to hold reusable Parameter Objects.
--- @field parameters table<string, ReferenceOr> A table mapping strings to Parameter reference objects.
--- A table to hold reusable Example Objects.
--- @field examples table<string, ReferenceOr> A table mapping strings to Example reference objects.
--- A table to hold reusable Request Body Objects.
--- @field request_bodies table<string, ReferenceOr> A table mapping strings to RequestBody reference objects.
--- A table to hold reusable Header Objects.
--- @field headers table<string, ReferenceOr> A table mapping strings to Header reference objects.
--- A table to hold reusable Security Scheme Objects.
--- @field security_schemes table<string, ReferenceOr> A table mapping strings to SecurityScheme reference objects.
--- A table to hold reusable Link Objects.
--- @field links table<string, ReferenceOr> A table mapping strings to Link reference objects.
--- A table to hold reusable Callback Objects.
--- @field callbacks table<string, ReferenceOr> A table mapping strings to Callback reference objects.
--- A table representing inline extensions to this object.
--- These extensions provide additional, custom data.
--- @field extensions table<string, any> A table mapping strings to JSON values representing additional extensions.


--- A table that maps strings to PathItems, representing callback paths.
---@alias Callback table<string, PathItem>

--- An alias for a table representing security requirements.
--- The keys of the table are security scheme names (strings), and the values are arrays (tables) of required scope strings.
--- @alias SecurityRequirement table<string, string[]>

--- An alias for a table representing operation callbacks.
--- The keys of the table are callback names or identifiers (strings), and the values are either references to Callback objects or the Callback objects themselves.
---@alias OperationCallbacks table<string, ReferenceOr<Callback>>

--- Describes the operations available on a single path.
--- A Path Item MAY be empty, due to ACL constraints.
--- The path itself is still exposed to the documentation
--- viewer but they will not know which operations and
--- parameters are available.
---@class PathItem
---@field summary string|nil # An optional, string summary, intended to apply to all operations in this path.
---@field description string|nil # An optional, string description, intended to apply to all operations in this path. CommonMark syntax MAY be used for rich text representation.
---@field get Operation|nil # A definition of a GET operation on this path.
---@field put Operation|nil # A definition of a PUT operation on this path.
---@field post Operation|nil # A definition of a POST operation on this path.
---@field delete Operation|nil # A definition of a DELETE operation on this path.
---@field options Operation|nil # A definition of an OPTIONS operation on this path.
---@field head Operation|nil # A definition of a HEAD operation on this path.
---@field patch Operation|nil # A definition of a PATCH operation on this path.
---@field trace Operation|nil # A definition of a TRACE operation on this path.
---@field servers Server[] # An alternative server array to service all operations in this path.
---@field parameters ReferenceOr<QueryParameter|PathParameter|HeaderParameter>[] # A list of parameters that are applicable for all the operations described under this path.
---@field extensions table<string, any> # Inline extensions to this object.

--- Holds the relative paths to the individual endpoints and
--- their operations. The path is appended to the URL from the
--- Server Object in order to construct the full URL. The Paths
--- MAY be empty, due to ACL constraints.
---@class Paths
---@field paths table<string, ReferenceOr<PathItem>> # A map of PathItems or references to them.
---@field extensions table<string, any> # Inline extensions to this object.

--- Describes a single API operation on a path.
---@class Operation
---@field tags string[] # A list of tags for API documentation control.
---@field summary string|nil # A short summary of what the operation does.
---@field description string|nil # A verbose explanation of the operation behavior. CommonMark syntax MAY be used for rich text representation.
---@field external_docs ExternalDocumentation|nil # Additional external documentation for this operation.
---@field operation_id string|nil # Unique string used to identify the operation.
---@field parameters ReferenceOr<QueryParameter|HeaderParameter|PathParameter>[] # A list of parameters that are applicable for this operation.
---@field request_body ReferenceOr<RequestBody>|nil # The request body applicable for this operation.
---@field responses Responses # REQUIRED. The list of possible responses as they are returned from executing this operation.
---@field callbacks table<string, Callback> # A map of possible out-of-band callbacks related to the parent operation.
---@field deprecated boolean # Declares this operation to be deprecated. Default value is false.
---@field security SecurityRequirement[]|nil # A declaration of which security mechanisms can be used for this operation.
---@field servers Server[] # An alternative server array to service this operation.
---@field extensions table<string, any> # Inline extensions to this object.

--- A container for the expected responses of an operation. The container maps
--- an HTTP response code to the expected response.
---@class Responses
---@field default ReferenceOr<Response>|nil # The documentation of responses other than the ones declared for specific HTTP response codes.
---@field responses table<StatusCode, ReferenceOr<Response>> # Maps HTTP status codes to their expected responses.
---@field extensions table<string, any> # Inline extensions to this object.

--- Represents an HTTP status code or a range of HTTP status codes.
---@class StatusCode
---@field Code fun(value: number): StatusCode # Represents a specific HTTP status code.
---@field Range fun(value: number): StatusCode # Represents a range of HTTP status codes.

--- Represents an API Key security scheme.
--- @class APIKeySecurityScheme
--- @field location string The location of the API key. Valid values are "query", "header", or "cookie".
--- @field name string The name of the header, query, or cookie parameter to be used.
--- @field description string|nil A short description for the security scheme. CommonMark syntax MAY be used for rich text representation.
--- @field extensions table<string, any> Inline extensions to this object.

--- Represents an HTTP security scheme.
--- @class HTTPSecurityScheme
--- @field scheme string The name of the HTTP Authorization scheme to be used in the Authorization header as defined in RFC7235.
--- @field bearer_format string|nil Optional format for bearer tokens.
--- @field description string|nil A short description for the security scheme. CommonMark syntax MAY be used for rich text representation.
--- @field extensions table<string, any> Inline extensions to this object.

--- Represents an OAuth2 security scheme.
--- @class OAuth2SecurityScheme
--- @field flows OAuth2Flows An object containing configuration information for supported flow types.
--- @field description string|nil A short description for the security scheme. CommonMark syntax MAY be used for rich text representation.
--- @field extensions table<string, any> Inline extensions to this object.

--- Represents an OpenID Connect security scheme.
--- @class OpenIDConnectSecurityScheme
--- @field open_id_connect_url string OpenId Connect URL to discover OAuth2 configuration values.
--- @field description string|nil A short description for the security scheme. CommonMark syntax MAY be used for rich text representation.
--- @field extensions table<string, any> Inline extensions to this object.

--- APIKeyLocation class enumerates valid locations for API keys.
--- @class APIKeyLocation
--- @field Query string Represents API key location in query parameters.
--- @field Header string Represents API key location in headers.
--- @field Cookie string Represents API key location in cookies.

--- OAuth2Flows class contains configuration for OAuth2 flow types.
--- @class OAuth2Flows
--- @field implicit ImplicitOAuth2Flow|nil Configuration for the OAuth Implicit flow.
--- @field password PasswordOAuth2Flow|nil Configuration for the OAuth Resource Owner Password flow.
--- @field client_credentials ClientCredentialsOAuth2Flow|nil Configuration for the OAuth Client Credentials flow.
--- @field authorization_code AuthorizationCodeOAuth2Flow|nil Configuration for the OAuth Authorization Code flow.
--- @field extensions table<string, any> Inline extensions to this object.

--- Represents the configuration for the OAuth Implicit flow.
--- @class ImplicitOAuth2Flow
--- @field authorization_url string The authorization URL for this flow.
--- @field refresh_url string|nil The URL for obtaining refresh tokens.
--- @field scopes table<string, string> Available scopes for the OAuth2 security scheme.
--- @field extensions table<string, any> Inline extensions to this object.

--- Represents the configuration for the OAuth Password flow.
--- @class PasswordOAuth2Flow
--- @field token_url string The token URL for this flow.
--- @field refresh_url string|nil The URL for obtaining refresh tokens.
--- @field scopes table<string, string> Available scopes for the OAuth2 security scheme.
--- @field extensions table<string, any> Inline extensions to this object.

--- Represents the configuration for the OAuth Client Credentials flow.
--- @class ClientCredentialsOAuth2Flow
--- @field token_url string The token URL for this flow.
--- @field refresh_url string|nil The URL for obtaining refresh tokens.
--- @field scopes table<string, string> Available scopes for the OAuth2 security scheme.
--- @field extensions table<string, any> Inline extensions to this object.

--- Represents the configuration for the OAuth Authorization Code flow.
--- @class AuthorizationCodeOAuth2Flow
--- @field authorization_url string The authorization URL for this flow.
--- @field token_url string The token URL for this flow.
--- @field refresh_url string|nil The URL for obtaining refresh tokens.
--- @field scopes table<string, string> Available scopes for the OAuth2 security scheme.
--- @field extensions table<string, any> Inline extensions to this object.

--- Describes a single request body.
--- @class RequestBody
--- @field description string|nil A brief description of the request body. This could contain examples of use. CommonMark syntax MAY be used for rich text representation.
--- @field content table<string, MediaType> REQUIRED. The content of the request body. The key is a media type or media type range and the value describes it. For requests that match multiple keys, only the most specific key is applicable (e.g., text/plain overrides text/*).
--- @field required boolean Determines if the request body is required in the request. Defaults to false.
--- @field extensions table<string, any> Inline extensions to this object.


--- Enum for representing different header styles as defined by various specifications.
--- @class HeaderStyle
--- Simple style parameters defined by RFC6570.
--- @field Simple string

--- Enum for representing different path styles as defined by various specifications.
--- @class PathStyle
--- Path-style parameters defined by RFC6570.
--- @field Matrix string
--- Label style parameters defined by RFC6570.
--- @field Label string
--- Simple style parameters defined by RFC6570.
--- @field Simple string

--- Enum for representing different cookie styles as defined by various specifications.
--- @class CookieStyle
--- Form style parameters defined by RFC6570.
--- @field Form string

--- Query parameters that are appended to the URL.
--- @class QueryParameter
--- @field parameter_data ParameterData Flattened parameter data.
--- @field allow_reserved boolean Determines whether the parameter value SHOULD allow reserved characters as defined by RFC3986 to be included without percent-encoding. Applies only to query parameters.
--- @field style QueryStyle Describes how the parameter value will be serialized depending on its type.
--- @field allow_empty_value boolean|nil Sets the ability to pass empty-valued parameters. Valid only for query parameters.

--- Header parameters expected as part of the request.
--- @class HeaderParameter
--- @field parameter_data ParameterData Flattened parameter data.
--- @field style HeaderStyle Describes how the parameter value will be serialized depending on its type.

--- Path parameters used together with Path Templating.
--- @class PathParameter
--- @field parameter_data ParameterData Flattened parameter data.
--- @field style PathStyle Describes how the parameter value will be serialized depending on its type.

--- Cookie parameters used to pass specific cookie values to the API.
--- @class CookieParameter
--- @field parameter_data ParameterData Flattened parameter data.
--- @field style CookieStyle Describes how the parameter value will be serialized depending on its type.


--- Describes a single operation parameter.
---
--- A unique parameter is defined by a combination of a name and location.
--- @class ParameterData
--- @field name string REQUIRED. The name of the parameter. Parameter names are case sensitive.
--- If `in` is "path", the name field MUST correspond to the associated path
--- segment from the path field in the Paths Object. See Path Templating for
--- further information.
--- If `in` is "header" and the name field is "Accept", "Content-Type" or
--- "Authorization", the parameter definition SHALL be ignored.
--- For all other cases, the name corresponds to the parameter name
--- used by the `in` property.
--- @field description string|nil A brief description of the parameter. This could
--- contain examples of use. CommonMark syntax MAY be
--- used for rich text representation.
--- @field required boolean Determines whether this parameter is mandatory.
--- If the parameter location is "path", this property
--- is REQUIRED and its value MUST be true. Otherwise,
--- the property MAY be included and its default value
--- is false.
--- @field deprecated boolean|nil Specifies that a parameter is deprecated and SHOULD
--- be transitioned out of usage.
--- @field format ParameterSchemaOrContent|string The format of the parameter as defined by the schema or content.
--- @field example table|nil An example of the parameter usage in JSON.
--- @field examples table<string, ReferenceOr<Example>>|nil Multiple examples of the parameter usage.
--- @field explode boolean|nil Whether or not the parameter should be exploded.
--- @field extensions table<string, any> Inline extensions to this object.

--- Represents an ExternalDocumentation Object that allows referencing an external resource for extended documentation.
---@class ExternalDocumentation
---@field description string | nil # A short description of the target documentation. Supports CommonMark syntax for rich text representation.
---@field url string # The URL for the target documentation (Required). Must be a valid URL format.
---@field extensions table<string, any> # Inline extensions to this object.

--- Represents a Tag Object which adds metadata to a single tag used by the Operation Object.
---@class Tag
---@field name string # The name of the tag (Required).
---@field description string | nil # A short description for the tag. Supports CommonMark syntax for rich text representation.
---@field external_docs ExternalDocumentation | nil # Additional external documentation for this tag.
---@field extensions table<string, any> # Inline extensions to this object.

--- @class ServerVariable
--- An object representing a Server Variable for server URL template substitution.
--- @field enumeration string[] # An enumeration of string values for limited set substitution options.
--- @field default string # REQUIRED. The default value to use for substitution if an alternate is not supplied.
--- @field description string|nil # An optional description for the server variable.
--- @field extensions table<string, any> # Inline extensions to this object.

--- @class Server
--- An object representing a Server.
--- @field url string # REQUIRED. A URL to the target host. Supports Server Variables and MAY be relative.
--- @field description string|nil # An optional string describing the host designated by the URL.
--- @field variables table<string, ServerVariable>|nil # A map between a variable name and its value for URL template substitution.
--- @field extensions table<string, any> # Inline extensions to this object.

--- The object provides metadata about the API.
--- The metadata MAY be used by the clients if needed,
--- and MAY be presented in editing or documentation generation tools for convenience.
--- @class Info
--- @field title string # REQUIRED. The title of the application.
--- @field description string|nil # A short description of the application. CommonMark syntax MAY be used for rich text representation.
--- @field terms_of_service string|nil # A URL to the Terms of Service for the API. MUST be in the format of a URL.
--- @field contact Contact|nil # The contact information for the exposed API.
--- @field license License|nil # The license information for the exposed API.
--- @field version string # REQUIRED. The version of the OpenAPI document, distinct from the OpenAPI Specification version or the API implementation version.
--- @field extensions table<string, any> # Inline extensions to this object.

--- License information for the exposed API.
--- @class License
--- @field name string # REQUIRED. The license name used for the API.
--- @field url string|nil # A URL to the license used for the API. MUST be in the format of a URL.
--- @field extensions table<string, any> # Inline extensions to this object.

--- @class Contact
--- Contact information for the exposed API.
--- @field name string|nil # The identifying name of the contact person/organization.
--- @field url string|nil # The URL pointing to the contact information. MUST be in the format of a URL.
--- @field email string|nil # The email address of the contact person/organization. MUST be in the format of an email address.
--- @field extensions table<string, any> # Inline extensions to this object.

--- Represents a property of type boolean.
---@class BooleanType
---@field enumeration (boolean | nil)[] # The enumeration of possible boolean values. Can contain true, false, or nil values.

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

--- Used to aid in serialization, deserialization, and validation when request bodies or response payloads
--- may be one of a number of different schemas.
---@class Discriminator
---@field propertyName string # REQUIRED. Name of the property in the payload holding the discriminator value
---@field mapping table<string, string> # Mappings between payload values and schema names or references
---@field extensions table<string, any> # Inline extensions to this object

--- The Link object represents a possible design-time link for a response.
--- The presence of a link does not guarantee the caller's ability to
--- successfully invoke it, rather it provides a known relationship and
--- traversal mechanism between responses and other operations.
---
--- Unlike dynamic links (i.e. links provided in the response payload),
--- the OAS linking mechanism does not require link information in the runtime response.
---
--- For computing links, and providing instructions to execute them,
--- a runtime expression is used for accessing values in an operation
--- and using them as parameters while invoking the linked operation.
--- @class Link
--- @field description string|nil A description of the link.
---        CommonMark syntax MAY be used for rich text representation.
--- @field operation LinkOperation Either a operationRef or operationId
--- @field request_body any|nil A literal value or {expression} to use as a request body
---        when calling the target operation.
--- @field parameters table<string, any> A map representing parameters to pass to an operation
---        as specified with operationId or identified via operationRef.
---        The key is the parameter name to be used, whereas the value
---        can be a constant or an expression to be evaluated and passed
---        to the linked operation. The parameter name can be qualified
---        using the parameter location [{in}.]{name} for operations
---        that use the same parameter name in different locations (e.g. path.id).
--- @field server Server|nil A server object to be used by the target operation.
--- @field extensions table<string, any> Inline extensions to this object.

--- Represents either an operation reference or operation ID.
--- @class LinkOperation
--- @field operation_ref string A relative or absolute reference to an OAS operation.
---        This field is mutually exclusive of the operationId field,
---        and MUST point to an Operation Object. Relative operationRef
---        values MAY be used to locate an existing Operation Object
---        in the OpenAPI definition.
--- @field operation_id string The name of an existing, resolvable OAS operation,
---        as defined with a unique operationId. This field is
---        mutually exclusive of the operationRef field.

--- Describes a single response from an API Operation, including design-time,
--- static links to operations based on the response.
--- @class Response
--- @field description string REQUIRED. A short description of the response.
---        CommonMark syntax MAY be used for rich text representation.
--- @field headers table<string, ReferenceOr<Header>> Maps a header name to its definition.
---        RFC7230 states header names are case insensitive.
---        If a response header is defined with the name "Content-Type",
---        it SHALL be ignored.
--- @field content table<string, MediaType> A map containing descriptions of potential response payloads.
---        The key is a media type or media type range and the value
---        describes it. For responses that match multiple keys,
---        only the most specific key is applicable. e.g. text/plain
---        overrides text/*
--- @field links table<string, ReferenceOr<Link>> A map of operations links that can be followed from the response.
---        The key of the map is a short name for the link, following
---        the naming constraints of the names for Component Objects.
--- @field extensions table<string, any> Inline extensions to this object.

--- Represents a reference or an item.
---@class ReferenceOr
---@field reference string # The reference string, applicable if it is a reference.
---@field item any

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

--- Represents a Schema object which encapsulates both schema data and kind.
--- @class Schema
--- @field schema_data SchemaData Contains basic schema properties.
--- @field schema_kind SchemaKind Specifies the kind of schema (e.g., type, composite).

--- Contains basic characteristics of a schema.
--- @class SchemaData
--- @field nullable boolean Indicates if the schema is nullable.
--- @field read_only boolean Indicates if the schema is read-only.
--- @field write_only boolean Indicates if the schema is write-only.
--- @field deprecated boolean Indicates if the schema is deprecated.
--- @field external_docs ExternalDocumentation|nil Link to external documentation.
--- @field example any|nil Example value compliant with the schema.
--- @field title string|nil Title of the schema.
--- @field description string|nil Description of the schema.
--- @field discriminator Discriminator|nil Discriminator for polymorphism.
--- @field default any|nil A default value compliant with the schema.
--- @field extensions table<string, any> Inline extensions to this object.

--- Enum representing different kinds of schemas.
--- @class SchemaKind
--- Represents a simple type schema.
--- @field [1] Type Simple type definition.
--- Represents a list of potential schemas where one must be valid.
--- @field one_of table<number, ReferenceOr|Schema> List of possible schemas.
--- Represents a combination of schemas where all must be valid.
--- @field all_of table<number, ReferenceOr|Schema> List of combined schemas.
--- Represents a list of schemas where any must be valid.
--- @field any_of table<number, ReferenceOr|Schema> List of alternative schemas.
--- Specifies a schema that should not be applicable.
--- @field not ReferenceOr|Schema Schema that should not match.
--- Represents any arbitrary schema.
--- @field any AnySchema Generic schema allowing any structure.

--- Enum representing different data types in a schema.
--- @class Type
--- @field STRING string    # Represents a string data type with specific attributes.
--- @field NUMBER string    # Represents a number data type, including floating-point numbers.
--- @field INTEGER string   # Represents an integer data type with its constraints.
--- @field OBJECT string    # Represents an object data type with properties and structure.
--- @field ARRAY string     # Represents an array data type with items of a specified type.
--- @field BOOLEAN string   # Represents a boolean data type, true or false.
Type = {}

Type.STRING = "string"
Type.NUMBER = "number"
Type.INTEGER = "integer"
Type.OBJECT = "object"
Type.ARRAY = "array"
Type.BOOLEAN = "boolean"

--- Represents different number formats.
--- @class NumberFormat
--- @field FLOAT string     # Floating-point format for numbers.
--- @field DOUBLE string    # Double precision format for numbers.
NumberFormat = {}

NumberFormat.FLOAT = "float"
NumberFormat.DOUBLE = "double"

--- Represents different string formats.
--- @class StringFormat
--- @field DATE string        # Date format for strings.
--- @field DATE_TIME string   # Date and time format for strings.
--- @field PASSWORD string    # Password format, used for secret fields.
--- @field BYTE string        # Byte format, representing base64 encoded data.
--- @field BINARY string      # Binary format, representing raw binary data.
StringFormat = {}

StringFormat.DATE = "date"
StringFormat.DATE_TIME = "date-time"
StringFormat.PASSWORD = "password"
StringFormat.BYTE = "byte"
StringFormat.BINARY = "binary"

--- Represents a string data type with additional validation attributes.
--- @class StringType
--- @field format StringFormat Format of the string.
--- @field pattern string Optional regex pattern to validate the string.
--- @field enumeration string[] Possible values for the string.
--- @field min_length number Minimum length of the string.
--- @field max_length number Maximum length of the string.

--- Represents a number data type with additional validation attributes.
--- @class NumberType
--- @field format NumberFormat Format of the number.
--- @field multiple_of number Optional requirement for the number to be a multiple of this value.
--- @field exclusive_minimum boolean Whether the minimum value is exclusive.
--- @field exclusive_maximum boolean Whether the maximum value is exclusive.
--- @field minimum number Minimum value of the number.
--- @field maximum number Maximum value of the number.
--- @field enumeration number[] Possible values for the number.

--- Represents different integer formats.
--- @class IntegerFormat
--- @field INT32 string  # Integer format for 32-bit integers.
--- @field INT64 string  # Integer format for 64-bit integers.
IntegerFormat = {}

IntegerFormat.INT32 = "int32"
IntegerFormat.INT64 = "int64"

--- Represents an integer data type with additional validation attributes.
--- @class IntegerType
--- @field format IntegerFormat Format of the integer.
--- @field multiple_of number Optional requirement for the integer to be a multiple of this value.
--- @field exclusive_minimum boolean Whether the minimum value is exclusive.
--- @field exclusive_maximum boolean Whether the maximum value is exclusive.
--- @field minimum number Minimum value of the integer.
--- @field maximum number Maximum value of the integer.
--- @field enumeration number[] Possible values for the integer.

--- Represents an object data type with structural attributes.
--- @class ObjectType
--- @field properties table<string, ReferenceOr<Schema>> Properties of the object.
--- @field required string[] List of required property names.
--- @field additional_properties AdditionalProperties Constraints on additional properties.
--- @field min_properties number Minimum number of properties.
--- @field max_properties number Maximum number of properties.

--- Represents an array data type with item constraints.
--- @class ArrayType
--- @field items ReferenceOr<Schema> Optional schema for items in the array.
--- @field min_items number Minimum number of items.
--- @field max_items number Maximum number of items.
--- @field unique_items boolean Whether items need to be unique.

--- Enum for specifying additional properties in an object.
--- @class AdditionalProperties
--- Indicates any type is allowed as an additional property.
--- @field Any boolean
--- Specifies a schema to validate additional properties.
--- @field Schema ReferenceOr<Schema>

---------------------------------------------------------------------------------------------------
--- Generic stack
--- @generic T
--- @class Stack<T>
--- @field items any[]
--- @field stackName string
Stack = {}
Stack.__index = Stack

function Stack.new(stackName)
    local instance = setmetatable({}, Stack)
    instance.stackName = stackName
    --- @generic T
    --- @type T[]
    instance.items = {}
    return instance
end

--- @generic T
--- @return T[]
function Stack:items()
    return self.items
end

--- Creates a shallow copy of the current stack
--- @generic T
--- @return Stack<T> # A new stack instance containing the same items
function Stack:clone()
    local newStack = Stack.new(self.stackName)

    for i, item in ipairs(self.items) do
        newStack.items[i] = item
    end

    return newStack
end

--- Checks whether the given item is in the stack.
--- @generic T
--- @param item T # The item to check for.
--- @return boolean # True if the item is in the stack, false otherwise.
function Stack:contains(item)
    for _, v in ipairs(self.items) do
        if v == item then
            return true
        end
    end
    return false
end

function Stack:push(item)
    table.insert(self.items, item)
    print(string.format([[
CONTEXT <- push to stack [%s], after
[
%s
]
]], self.stackName, tableToString(self)))
    return item
end

function Stack:pushSilent(item)
    table.insert(self.items, item)
    return item
end

--- Pushes all elements from the given table onto the stack.
--- @generic T
--- @param elements T[] # A table containing elements to be pushed onto the stack
function Stack:pushAll(elements)
    for _, element in ipairs(elements) do
        table.insert(self.items, element)
    end
end

--- return last element and delete it from stack
--- @generic T
--- @return T
function Stack:pop()
    if #self.items == 0 then
        error(string.format("Stack [%s] is empty", self.stackName))
    end
    print(string.format([[
CONTEXT -> pop from stack [%s], before
[
%s
]
]], self.stackName, tableToString(self)))
    local item = table.remove(self.items)
    return item
end

--- @return Property
function Stack:popProperty()
    local property = self:pop()
    assert(property:instanceOf(Property), "Found not a Property")
    return property
end

--- @generic T
--- @return T
function Stack:peek()
    if #self.items == 0 then
        return nil
    end
    local item = self.items[#self.items]
    print(string.format([[
CONTEXT -> peek from stack [%s]
[
%s
]
]], self.stackName, tableToString(self)))
    return item
end

--- Retrieves, but does not remove, the head of this stack.
--- This method differs from peek only in that it throws an error if this stack is empty.
--- @generic T
--- @return T # The head of this stack
function Stack:element()
    if #self.items == 0 then
        error(string.format("Stack [%s] is empty.", self.stackName))
    end
    local item = self.items[#self.items]
    print(string.format([[
CONTEXT -> element from stack [%s]
[
%s
]
]], self.stackName, tableToString(self)))
    return item
end

--- @return ObjectModel
function Stack:elementObjectModel()
    local value = self:element()
    assert(value:instanceOf(ObjectModel), "Found not a ObjectModel")
    return value
end

--- Retrieves, but does not remove, the penultimate element of this stack.
--- This method return nil if the stack has fewer than two elements.
--- @generic T
--- @return T|nil # The penultimate element of this stack
function Stack:penultimate()
    if #self.items < 2 then
        return nil
    end
    local item = self.items[#self.items - 1]
    print(string.format([[
CONTEXT -> penultimate from stack [%s]
[
%s
]
]], self.stackName, tableToString(self)))
    return item
end

--- Method to apply a given function to each element in the stack.
--- @generic T
--- @param action fun(it:T) # A function to be applied to each element in the stack.
function Stack:forEach(action)
    for _, item in ipairs(self.items) do
        action(item)
    end
end

--- Method to create a new stack by applying a given function to each element in the current stack.
--- @generic T,R
--- @param transform fun(it:T):R # A function to transform each element in the stack.
--- @return Stack<T> # A new stack containing the transformed elements.
function Stack:map(transform)
    local newStack = Stack.new()
    self:forEach(function(item)
        newStack:pushSilent(transform(item))
    end)
    return newStack
end

--- Method to apply a given function to each property in the stack.
---@param action fun(property: Property) # A function to be applied to each element in the stack.
function Stack:forEachProperty(action)
    for _, item in ipairs(self.items) do
        assert(item:instanceOf(Property), "Not a Property type of:" .. tableToString(item))
        action(item)
    end
end

--- Method to reduce the elements of the stack to a single value.
--- @generic T
--- @param accumulator any # The initial value for the reduction.
--- @param reducer function # A function that takes the accumulator and an element, and returns a new accumulator.
--- @return T # The final reduced value.
function Stack:reduce(accumulator, reducer)
    for _, item in ipairs(self.items) do
        accumulator = reducer(accumulator, item)
    end
    return accumulator
end

function Stack:isEmpty()
    return #self.items == 0
end

function Stack:clear()
    self.items = {}
end

function Stack:size()
    return #self.items
end

--- @param separator string?
--- @return string # A string containing all elements concatenated with separator
function Stack:concat(separator)
    separator = separator or ""
    return table.concat(self.items, separator)
end

--- Concatenates all elements of the stack into a single string, with each element's first letter capitalized.
--- @param stack table # The stack whose elements will be concatenated. Must have an `items` array.
--- @param separator string? # The separator between elements (defaults to an empty string).
--- @return string? # The concatenated string with each element's first letter capitalized, or `nil` if the stack is empty.
function concatStackCapitalized(stack, separator)
    separator = separator or "" -- Use an empty string if no separator is provided
    if (#stack.items == 0) then
        return nil
    end

    local result = ""
    for i, item in ipairs(stack.items) do
        local capitalized = item:gsub("^%l", string.upper)
        result = result .. (i > 1 and separator or "") .. capitalized
    end

    return result
end

--- Concatenates all elements of the stack into a single string,
--- capitalizing the first letter of each word and removing underscores
--- and non-alphanumeric characters.
---
--- @param stack Stack # The stack whose elements will be concatenated. Must have an `items` array.
--- @param separator string? # The separator between elements (defaults to an empty string).
--- @return string # The concatenated string with processed elements, or `nil` if the stack is empty.
function concatStackCleanCapitalized(stack, separator)
    separator = separator or "" -- Use an empty string if no separator is provided
    assert(#stack.items, "Stack is empty")

    local result = ""
    for i, item in ipairs(stack.items) do
        assert(type(item) == "string", "Non string value for concatenation")
        -- Remove underscores and non-alphanumeric characters, then capitalize first letter
        local cleaned_item = item:gsub("[^%w%d]", ""):gsub("_", "")     -- Remove non-alphanumeric and underscores
        local capitalized_item = cleaned_item:gsub("^%l", string.upper) -- Capitalize first letter
        result = result .. (i > 1 and separator or "") .. capitalized_item
    end

    return result
end

--- Merges two tables if they have unique keys.
--- This function merges two tables into one if they do not have any overlapping keys.
--- If a duplicate key is found, an error is thrown.
--- @param left table The first table to merge.
--- @param right table The second table to merge.
--- @return table The merged table containing all unique keys from both input tables.
--- @throws Error if a duplicate key is found in both tables.
function mergeTablesWithUniqueKeys(left, right)
    local result = {}

    for key, value in pairs(left) do
        result[key] = value
    end

    for key, value in pairs(right) do
        if result[key] ~= nil then
            error("Duplicate key [" .. tostring(key) .. "] with value [" .. tostring(value) .. "]")
        end
        result[key] = value
    end

    return result
end

--- Merges two arrays into one, excluding duplicate elements.
--- @param left table # The first array.
--- @param right table # The second array.
--- @return table # A merged array containing unique elements from both arrays.
function mergeArraysUnique(left, right)
    local result = {}
    local seen = {}

    for _, value in ipairs(left) do
        if not seen[value] then
            table.insert(result, value)
            seen[value] = true
        end
    end

    for _, value in ipairs(right) do
        if not seen[value] then
            table.insert(result, value)
            seen[value] = true
        end
    end

    return result
end

--- Class for storing variables across scripts with loggable access manner for all chain of models
--- @class GlobalContext
--- @field generatedClasses Stack # stack of names of generated classes
--- @field names Stack # stack of names of processed schemas
--- @field savedNames Stack # stack of names for temporary save when reference processing executed
--- @field values Stack # stack of models or properties in processing
GlobalContext = {}
GlobalContext.__index = GlobalContext

--- Constructor to create a new instance of the GlobalContext class.
--- @return GlobalContext # A new instance of the GlobalContext class.
function GlobalContext:new()
    --- @class GlobalContext
    local instance = setmetatable({}, GlobalContext)
    --- @type Stack
    instance.names = Stack.new("modelsNames")
    --- @type Stack
    instance.savedNames = Stack.new("savedNames")
    --- @type Stack
    instance.values = Stack.new("values")
    --- @type Stack
    instance.generatedClasses = Stack.new("generatedClasses")
    return instance
end

--- global accessed context within scripts
GLOBAL_CONTEXT = GlobalContext:new()

--- Function to check if a table contains a specific value.
---@param tbl table # The table to search in.
---@param value any # The value to search for.
---@return boolean # Returns true if the table contains the value, false otherwise.
function table.contains(tbl, value)
    if tbl ~= nil then
        for _, v in ipairs(tbl) do
            if v == value then
                return true
            end
        end
    end

    return false
end

--- Print line
function printBreak()
    print(
        "-------------------------------------------------------------------------------------------------------------------")
end

--- Print table to console
--- @param t table # table for conversion
--- @param indent number|nil # level of incapsulation
function printTable(t, indent)
    indent = indent or 4

    if t == NULL then
        print(string.rep(" ", indent) .. "Argument is NULL!")
        return
    end

    if t == nil then
        print(string.rep(" ", indent) .. "Argument is nil!")
        return
    end

    if type(t) ~= "table" then
        print(string.rep(" ", indent) .. "Argument is not a table type with value [" .. tostring(t) .. "]")
        return
    end
    if isTableEmpty(t) then
        -- print(string.rep(" ", indent) .. "empty")
    else
        for key, value in pairs(t) do
            local formatting = string.rep(" ", indent) .. string.format("%s", key) .. ":"
            if type(value) == "table" then
                print(formatting)
                printTable(value, indent + 2)
            else
                print(formatting .. " " .. tostring(value))
            end
        end
    end
end

--- Table into string
--- @param tbl table? # table for conversion
--- @param indent number? # level of incapsulation
--- @return string # string interpretation of table
function tableToString(tbl, indent)
    local result = {}
    local rootCall = indent == nil
    if rootCall then
        table.insert(result, "```yaml")
    end

    ---@type number
    indent = indent or 0

    if tbl == NULL then
        return string.rep(" ", indent) .. "Argument is NULL!"
    end

    if tbl == nil then
        return string.rep(" ", indent) .. "Argument is nil!"
    end

    if type(tbl) ~= "table" then
        return string.rep(" ", indent) .. "Argument is not a table type with value [" .. tostring(tbl) .. "]"
    end

    local spacing = string.rep(" ", indent)

    for key, value in pairs(tbl) do
        local formatting = spacing .. string.format("%s", key) .. ":"
        if type(value) == "table" then
            if isTableEmpty(value) then
                table.insert(result, formatting .. " empty")
            else
                table.insert(result, formatting)
                table.insert(result, tableToString(value, indent + 2))
            end
        else
            table.insert(result, formatting .. " " .. tostring(value))
        end
    end

    if rootCall then
        return table.concat(result, "\n") .. "\n```"
    else
        return table.concat(result, "\n")
    end
end

---@param tbl table # source table
---@return boolean # true if table empty
function isTableEmpty(tbl)
    local count = 0
    for _ in pairs(tbl) do
        count = count + 1
    end

    return count == 0
end

function printCalls()
    print("The call stack, markdown links (#link-x) work and are clickable:")
    printTable(CALLS)
end

function errorHandler(err)
    printBreak()
    print("Error handled")
    print(err)
    print("Names stack")
    printTable(GLOBAL_CONTEXT.names)
    print("Saved names stack")
    printTable(GLOBAL_CONTEXT.savedNames)
    print("Models stack")
    printTable(GLOBAL_CONTEXT.values)
    printCalls()
    return err
end

-- local function callWithErrorHandler(callable, args)
-- local status, result = xpcall(callable, errorHandler, table.unpack(args))
-- if status then
--     return result
-- else
--     error(result)
-- end
-- end

local lastShowedModelName

--- Function decorator for logging
--- @param funcName string # name of called function
--- @param mainFunc function # main function with arguments from Rust code
--- @param localIndent integer? # indent for function call logger
function functionCallAndLog(funcName, mainFunc, localIndent)
    return function(...)
        local args = { ... }
        local callNumber = tostring(#CALLS)
        local callId = args[#args]

        if callId == NULL then
            callId = "no-id"
        end

        local currentModelName = concatStackCapitalized(GLOBAL_CONTEXT.names, "->")

        local appendModelName = ""
        if lastShowedModelName ~= currentModelName then
            if currentModelName ~= nil and #currentModelName ~= 0 then
                appendModelName = " : started " .. currentModelName
            else
                if lastShowedModelName ~= nil and #lastShowedModelName ~= 0 then
                    appendModelName = " : finished " .. lastShowedModelName
                else
                    appendModelName = " : empty"
                end
            end
            lastShowedModelName = currentModelName
        end

        local callLink = string.format("[%s](#link-%s)[%s]", callNumber, callNumber,
            string.rep(" ", GLOBAL_CONTEXT.values:size()))

        table.insert(CALLS, callLink .. funcName .. " -> {" .. callId .. "}" .. appendModelName)
        print("# link-" .. callNumber .. "\nCALL <- [" .. funcName .. "]")

        if localIndent ~= nil then
            INDENT = INDENT + localIndent
        end

        for i, v in ipairs(args) do
            local indent = "    "
            if type(v) == "table" then
                if isTableEmpty(v) then
                    print(indent .. "arg" .. i .. " = [table] empty")
                else
                    print(indent .. "arg" .. i .. " = [table]")
                    printTable(v, DEFAULT_PRINT_ARGS_INDENT)
                end
            else
                print(indent .. "arg" .. i .. " = " .. tostring(v))
            end
        end

        local result = mainFunc(...)

        if type(result) == "table" then
            if isTableEmpty(result) then
                print("RETURN <- [" .. funcName .. "] [table] empty\n")
            else
                print("RETURN <- [" .. funcName .. "] [table]")
                printTable(result, 8)
                print()
            end
        else
            print("RETURN <- [" .. funcName .. "] " .. tostring(result) .. "\n")
        end

        for _, v in ipairs(args) do
            if type(v) == "table" and v[Extensions.DEBUG_STOP] then
                error("Found debug stop in spec")
            end
        end
        return result
    end
end

--- For WriteOperation it is mode of operation on disk
--- @class WriteMode
--- @field APPEND string # Add something at end of file, if file does not exists create it
--- @field PREPEND string # Add something at start of file, if file does not exists create it
--- @field REMOVE string # Remove some model file if it exists or no op
WriteMode = {}

WriteMode.APPEND = "APPEND"
WriteMode.PREPEND = "PREPEND"
WriteMode.REMOVE = "REMOVE"

--- Enum emulation for predefined extensions
--- @class Extensions
--- @field MODEL_NAME string #
--- @field PROPERTY_NAME string #
--- @field ADDITIONAL_PROPERTY_NAME string #
--- @field ADDITIONAL_PROPERTY_MODEL_NAME string #
--- @field DEBUG_STOP string # if found, then stop
--- @field VARIANT string # code variant for generation
Extensions = {}

Extensions.MODEL_NAME = "x-ot-model-name"
Extensions.PROPERTY_NAME = "x-ot-property-name"
Extensions.ADDITIONAL_PROPERTY_NAME = "x-ot-additional-property-name"
Extensions.ADDITIONAL_PROPERTY_MODEL_NAME = "x-ot-additional-property-model-name"
Extensions.DEBUG_STOP = "x-ot-debug-stop"
Extensions.VARIANT = "x-ot-variant"

--- Extracts the last component from a string delimited by '/'.
--- @param reference string # representing a path where components are separated by '/'.
--- @return string # The last component of the string, or `nil` if the string is empty.
function lastReferencePart(reference)
    local last_component = nil
    for component in string.gmatch(reference, "[^/]+") do
        last_component = component
    end
    return last_component
end

---convert null value to nil value if need
---@param value any|null # it possible be NULL
---@return any|nil
function nullableAsNillable(value)
    if value == NULL then
        return nil
    else
        return value
    end
end

--- Output text and target file name to write
--- @class WriteOperation
--- @field code string generated code
--- @field file string output file name
--- @field mode string output file name
WriteOperation = {}
WriteOperation.__index = WriteOperation

--- @param code string # produced code
--- @param modelName string # output model name to construct file name
function WriteOperation.new_prepend(code, modelName)
    if modelName == nil or modelName == NULL or modelName:len() == 0 then
        error("empty modelName")
    end
    local instance = setmetatable({}, WriteOperation)
    instance.code = code
    instance.file = modelName .. ".java"
    instance.mode = WriteMode.PREPEND
    return instance
end

--- @param modelName string # output model name to construct file name
function WriteOperation.new_remove(modelName)
    if modelName == nil or modelName == NULL or modelName:len() == 0 then
        error("empty modelName")
    end
    local instance = setmetatable({}, WriteOperation)
    instance.code = nil
    instance.file = modelName .. ".java"
    instance.mode = WriteMode.REMOVE
    return instance
end

--- @param code Stack<string> # produced code
--- @param fileName string # data for generate file name
function WriteOperation.new_from_code(code, fileName)
    assert(not is_string_blank(fileName), "empty fileName")
    local instance = setmetatable({}, WriteOperation)
    instance.code = code:concat()
    instance.file = fileName
    instance.mode = WriteMode.APPEND
    return instance
end

--- @param code string # produced code
--- @param modelName string # data for generate file name
function WriteOperation.new_append(code, modelName)
    if modelName == nil or modelName == NULL or modelName:len() == 0 then
        error("empty modelName")
    end
    local instance = setmetatable({}, WriteOperation)
    instance.code = code
    instance.file = modelName .. ".java"
    instance.mode = WriteMode.APPEND
    return instance
end

--- @param code string # produced code
--- @param modelName string # data for generate file name
--- @param mode string # APPEND or PREPEND code to target file
function WriteOperation.new_with_mode(code, modelName, mode)
    if modelName == nil or modelName == NULL or modelName:len() == 0 then
        error("empty modelName")
    end
    local instance = setmetatable({}, WriteOperation)
    instance.code = code
    instance.file = modelName .. ".java"
    instance.mode = mode
    return instance
end

--- Create new write operation from old to new model file
--- @param modelName string # data for generate file name
--- @param writeOperation WriteOperation # produced code
function WriteOperation.from(writeOperation, modelName)
    if modelName == nil or modelName == NULL or modelName:len() == 0 then
        error("empty modelName")
    end
    local instance = setmetatable({}, WriteOperation)
    instance.code = writeOperation.code
    instance.file = modelName .. ".java"
    instance.mode = writeOperation.mode
    return instance
end

--- Create new array of write operations with updated file names
--- @param writeOperations WriteOperation[] # Array of WriteOperation objects
--- @param modelName string # Data for generating file name
--- @return WriteOperation[] # New array of WriteOperation objects
function adaptWriteOperations(writeOperations, modelName)
    local newWriteOperations = {}

    for _, writeOperation in ipairs(writeOperations) do
        local newOperation = WriteOperation.from(writeOperation, modelName)
        table.insert(newWriteOperations, newOperation)
    end

    return newWriteOperations
end

--- Checks if a string value is nil, null, or empty.
---
--- @param value string|nil|null The string value to check.
--- @return boolean Returns true if the value is nil, null, or empty, false otherwise.
function is_string_blank(value)
    if value == nil or value == NULL then
        return true
    end
    if type(value) ~= "string" then
        error("Not a string")
    end
    if string.len(value) == 0 then
        return true
    end
    if string.lower(value) == "null" then
        return true
    end
    return false
end

--- @class Value
--- @field className string? # Type, described by this model, String, CustomGenericModel
--- @field modelName string # just for logs information (ModelBase,ObjectModel, Property...)
--- @field descriptor table # descriptor from spec
--- @field extensions table # extensions from descriptor, just for usability
Value = {}
Value.__index = Value

--- @param className string?
--- @param modelName string
--- @param descriptor table
--- @param extensions table
function Value:new(className, modelName, descriptor, extensions)
    local instance = setmetatable({}, self) --- child set as metatable for parent instance
    instance.className = className
    instance.modelName = modelName
    instance.descriptor = descriptor
    instance.extensions = extensions
    return instance
end

--- Method to determinate type of model
--- @param clazz table # Class
--- @return boolean # true if this model is instance of that class
function Value:instanceOf(clazz)
    return self ~= nil and clazz ~= nil and getmetatable(self) == clazz
end

--- @param extensions table
--- @return Stack<string>
function Value:getCode(extensions)
    return Stack.new("unknown value code stack")
end

--- @class ObjectModel: Value
--- @field required string[] # list of required properties
--- @field properties Stack<Property> # list of properties
ObjectModel = {}
ObjectModel.__index = ObjectModel
setmetatable(ObjectModel, { __index = Value })

--- @param className string?
--- @param required string[] # list of required properties names
--- @param descriptor table
--- @param extensions table
--- @return ObjectModel
function ObjectModel.new(className, required, descriptor, extensions)
    --- @class ObjectModel
    local instance = Value.new(ObjectModel, className, "ObjectModel", descriptor, extensions)
    instance.required = required
    instance.properties = Stack.new(className .. "->properties")
    return instance
end

--- Method to get property required status
--- @param propertyName string # Type of last parent
--- @return boolean # required property (true) or not (false)
function ObjectModel:isPropertyRequired(propertyName)
    local required = table.contains(self.required, propertyName)
    print("\nCONTEXT <- for model [" .. self.className .. "] and property [" ..
        propertyName .. "] get required status as [" .. tostring(required) .. "]")
    return required
end

--- Method to add model
--- @param property Property # Required model name
--- @return Property # created property
function ObjectModel:addProperty(property)
    --- @type Property
    for _, item in ipairs(self.properties.items) do
        if item.name == property.name then
            error(string.format("Duplicate property name [%s]", property.name))
        end
    end
    self.properties:push(property)
    return property
end

--- Merges two ObjectModel instances into a new ObjectModel.
--- @param left ObjectModel # The first ObjectModel instance.
--- @param right ObjectModel # The second ObjectModel instance.
--- @return ObjectModel # A new ObjectModel instance containing the merged data.
function ObjectModel.__add(left, right)
    assert(left:instanceOf(ObjectModel), "Left parameter must be an ObjectModel")
    assert(right:instanceOf(ObjectModel), "Right parameter must be an ObjectModel")
    assert(left.className == right.className, "Different class names")

    local mergedModel = ObjectModel.new(
        left.className .. "_" .. right.className,
        mergeArraysUnique(left.required, right.required),
        mergeTablesWithUniqueKeys(left.descriptor, right.descriptor),
        mergeTablesWithUniqueKeys(left.extensions, right.extensions)
    )

    for _, property in ipairs(left.properties.items) do
        mergedModel:addProperty(property)
    end

    for _, property in ipairs(right.properties.items) do
        local exists = false
        for i, existingProperty in ipairs(mergedModel.properties.items) do
            if existingProperty.name == property.name then
                mergedModel.properties.items[i] = existingProperty + property
                exists = true
                break
            end
        end
        if not exists then
            mergedModel:addProperty(property)
        end
    end

    return mergedModel
end

--- @param extensions table
--- @return Stack<string>
function ObjectModel:getCode(extensions)
    local codeVariant = CODE.getVariant(extensions[Extensions.VARIANT])

    local classFileName = codeVariant:getClassFileName(self.className)

    --- @type Stack<string>
    local code = Stack.new("object [" .. classFileName .. "] code")

    self.properties:forEachProperty(
        function(it)
            code:pushAll(it:getIncludes().items)
        end)

    code:push(codeVariant:getClassHeader(self.className))

    self.properties:forEachProperty(
        function(it)
            code:pushAll(it:getCode(extensions).items)
        end)

    code:push(codeVariant:getClassFooter())

    print("Generated code")
    printTable(code.items)

    return code
end

--- @class Property: Value
--- @field name string
--- @field includes Stack<string> # includes for this type of property and all markers
--- @field markers Stack<string> # annotations for this property
--- @field methods Stack<string> # access methods for property
Property = {}
Property.__index = Property
setmetatable(Property, { __index = Value })

--- @param propertyName string # name of property
--- @param className string # class name for property value
--- @param descriptor table # descriptor from spec
--- @param extensions table # extensions from spec
--- @return Property
function Property.new(propertyName, className, descriptor, extensions)
    local name = extensions[Extensions.PROPERTY_NAME] or propertyName

    if not name then
        error("Model name is missing: neither 'propertyName' nor '" ..
            Extensions.PROPERTY_NAME .. "' in extensions is provided.")
    end

    --- @class Property
    local newProperty = Value.new(Property, className, "Property", descriptor, extensions)

    newProperty.name = name
    newProperty.includes = Stack.new(className .. "->includes")
    newProperty.markers = Stack.new(className .. "->markers")
    newProperty.methods = Stack.new(className .. "->methods")
    return newProperty
end

---@param include string?
function Property:addInclude(include)
    if include ~= nil then
        if not self.includes:contains(include) then
            self.includes:push(include)
        end
    end
end

--- @return Stack<string>
function Property:getIncludes()
    return self.includes:clone()
end

---@param marker string?
function Property:addMarker(marker)
    if marker ~= nil then
        if not self.markers:contains(marker) then
            self.markers:push(marker)
        else
            error("Duplicate marker in property")
        end
    end
end

---@param method string?
function Property:addMethod(method)
    if method ~= nil then
        if not self.methods:contains(method) then
            self.methods:push(method)
        else
            error("Duplicate method in property")
        end
    end
end

---@param left Property
---@param right Property
function Property.__add(left, right)
    assert(left:instanceOf(Property), "Left operand is not Property")
    assert(right:instanceOf(Property), "Right operand is not Property")
    assert(left.name == right.name, "Different properties names")
    assert(left.className == right.className, "Different properties classNames")

    local newProp = Property.new(left.name,
        left.className,
        mergeTablesWithUniqueKeys(left.descriptor, right.descriptor),
        mergeTablesWithUniqueKeys(left.extensions, right.extensions)
    )

    newProp.includes = left.includes:clone()
    for _, inc in ipairs(right.includes.items) do
        newProp:addInclude(inc)
    end

    newProp.markers = left.markers:clone()
    for _, marker in ipairs(right.markers.items) do
        newProp:addMarker(marker)
    end

    newProp.methods = left.methods:clone()
    for _, method in ipairs(right.methods.items) do
        newProp:addMethod(method)
    end

    return newProp
end

--- @param extensions table
--- @return Stack<string>
function Property:getCode(extensions)
    local code = Stack.new("Property " .. self.name .. " code")
    self.markers:forEach(
        function(it)
            code:push(it)
        end)
    code:push("public " .. self.className .. " " .. self.name .. ";")
    return code
end

--- Replaces placeholders in the string with corresponding values from a table.
--- Placeholders must be in the format ${key} in string.
--- @param str string # The string containing placeholders.
--- @param parameters table # A table containing key-value pairs for interpolation.
--- @return string # A new string where placeholders have been replaced by their corresponding values.
function interpolate(parameters, str)
    return (str:gsub("($%b{})", function(w) return parameters[w:sub(3, -2)] or w end))
end

--- Removes leading spaces from a multiline string.
--- Finds the minimum number of leading spaces present across all lines and removes that amount of spaces
--- from the beginning of each line (as kotlin trimIndent()).
--- @param str string # The multiline string to process.
--- @return string # A new string with leading spaces removed.
function trimIndent(str)
    local trimmedLines = {}

    local minIndent = math.huge
    for line in str:gmatch("[^\r\n]+") do
        local leadingSpaces = line:match("^%s+")
        if leadingSpaces and #leadingSpaces < minIndent then
            minIndent = #leadingSpaces
        elseif not leadingSpaces then
            minIndent = 0
            break
        end
    end

    for line in str:gmatch("[^\r\n]+") do
        table.insert(trimmedLines, line:sub(minIndent + 1))
    end

    return table.concat(trimmedLines, "\n")
end

--- Formats a string using string.format, then trims the first newline character and removes trailing indentations.
-- The function processes a formatted string by applying specific trim operations:
-- 1. Removes the first newline character from the formatted result.
-- 2. Removes the last line if it consists only of spaces.
-- 3. Removes leading spaces from all lines, subtracting the length of the indentation of the last line.
-- @param fmt The format string (same as string.format).
-- @param ... The values to be formatted into the string.
-- @return A new string with processed formatting and trimmed indents.
function formatAndTrimIndent(fmt, ...)
    local formattedStr = string.format(fmt, ...)

    -- Remove the first newline character
    if formattedStr:sub(1, 1) == "\n" then
        formattedStr = formattedStr:sub(2)
    end

    -- Split the string by lines
    local lines = {}
    for line in formattedStr:gmatch("[^\r\n]*") do
        table.insert(lines, line)
    end

    -- Check if the last line consists only of spaces
    local lastLine = lines[#lines]
    if lastLine:match("^%s*$") then
        table.remove(lines, #lines)
    end

    -- Find the leading spaces of the last line
    local lastLineIndent = lastLine:match("^%s*")
    local lastLineIndentLen = #lastLineIndent

    -- Trim the leading spaces from each line based on last line's indent length
    for i, line in ipairs(lines) do
        lines[i] = line:sub(lastLineIndentLen + 1)
    end

    return table.concat(lines, "\n")
end

--- Returns a concatenated table from multiple input tables
--- @vararg table # tables to concatenate
--- @return table # result concatenated table
function concatTables(...)
    local result = {}
    for i = 1, select("#", ...) do
        local tbl = select(i, ...)
        for _, v in ipairs(tbl) do
            table.insert(result, v)
        end
    end
    return result
end

--- This script is called first, at the beginning of all processing. It outputs the value of all parameters
--- passed to the script either from the OpenAPI specification or from the command line. Command line
--- parameters take precedence and override the specification parameters. Parameters are stored in the
--- global variable `targetParameters` created by the translator (Rust code) in the Lua context
--- @param callId string? # some useful identifier of this visitor call
local function prelude(callId)
    print("    targetParameters type: " .. type(TARGET_PARAMETERS))
    print("    targetParameters value:")
    printTable(TARGET_PARAMETERS)
end

return functionCallAndLog("prelude", prelude)
