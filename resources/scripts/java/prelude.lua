--- This section contains all global functions and variables that are created before the visitors
--- start working.

--- TYPES -----------------------------------------------------------------------------------------------

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

--- Allows referencing an external resource for extended documentation.
---@class ExternalDocsDescriptor
---@field description string | nil # A short description of the target documentation
---@field url string # REQUIRED. URL for the target documentation
---@field extensions table<string, any> # Inline extensions to this object

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
--- Represents a string data type with specific attributes.
--- @field String StringType
--- Represents a number data type, including floating-point numbers.
--- @field Number NumberType
--- Represents an integer data type with its constraints.
--- @field Integer IntegerType
--- Represents an object data type with properties and structure.
--- @field Object ObjectType
--- Represents an array data type with items of a specified type.
--- @field Array ArrayType
--- Represents a boolean data type, true or false.
--- @field Boolean BooleanType

--- Represents different number formats.
--- @class NumberFormat
--- Floating-point format for numbers.
--- @field Float NumberFormat
--- Double precision format for numbers.
--- @field Double NumberFormat

--- Represents different string formats.
--- @class StringFormat
--- Date format for strings.
--- @field date StringFormat
--- Date and time format for strings.
--- @field date_time StringFormat
--- Password format, used for secret fields.
--- @field password StringFormat
--- Byte format, representing base64 encoded data.
--- @field byte StringFormat
--- Binary format, representing raw binary data.
--- @field binary StringFormat

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
--- Integer format for 32-bit integers.
--- @field Int32 IntegerFormat
--- Integer format for 64-bit integers.
--- @field Int64 IntegerFormat
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

--- Class for storing variables across scripts with loggable access manner for all chain of models
--- @class GlobalContext
GlobalContext = {}
GlobalContext.__index = GlobalContext

--- Constructor to create a new instance of the GlobalContext class.
--- @return GlobalContext # A new instance of the GlobalContext class.
function GlobalContext:new()
    local instance = setmetatable({}, GlobalContext)
    --- @type table<string,Model>
    local models = {}
    --- For parents who can send information about self type to childs
    --- @type string[]
    local parentsTypes = {}
    --- Children can send information to parents about they model name
    --- @type string[]
    local childrenModelNames = {}

    --- Requiered properties for model
    --- @type table<string,string[]>
    local requiredProperties = {}

    --- Method to add required properties for model
    --- @param setter string # The name of the script who set the value.
    --- @param modelName string # model name with required properties
    --- @param modelRequiredProperties string[]|nil # Type of last parent
    function GlobalContext:setRequiredProperties(setter, modelName, modelRequiredProperties)
        requiredProperties[modelName] = modelRequiredProperties
        print("\nCONTEXT <- [" ..
            setter ..
            "] add required properties list for model [" ..
            modelName .. "] as [\n" .. tableToString(modelRequiredProperties) .. "\n]")
    end

    --- Method to get property required status
    --- @param getter string # The name of the script who set the value.
    --- @param modelName string # model name with required properties
    --- @param propertyName string # Type of last parent
    --- @return boolean # required property (true) or not (false)
    function GlobalContext:isPropertyRequired(getter, modelName, propertyName)
        local required = table.contains(requiredProperties[modelName], propertyName)
        print("\nCONTEXT <- [" ..
            getter ..
            "] for model [" .. modelName .. "] and property [" ..
            propertyName .. "] get required status as [" .. tostring(required) .. "]")
        return required
    end

    --- Method to add last parent type
    --- @param setter string # The name of the script who set the value.
    --- @param parentType string # Type of last parent
    function GlobalContext:addParentType(setter, parentType)
        table.insert(parentsTypes, parentType)
        print("\nCONTEXT <- [" ..
            setter ..
            "] add last parent type as [" .. parentType .. "] full chain is [\n" .. tableToString(parentsTypes) .. "\n]")
    end

    --- Method to get last parent type
    --- @param getter string # The name of the script who set the value.
    --- @return string? # last parent type
    function GlobalContext:getLastParentType(getter)
        local lastParentType = parentsTypes[#parentsTypes]
        print("\nCONTEXT -> [" ..
            getter ..
            "] get last parent type as [" ..
            lastParentType .. "] full chain is [\n" .. tableToString(parentsTypes) .. "\n]")
        return lastParentType
    end

    --- Method to drop last parent type
    --- @param setter string # The name of the script who set the value.
    function GlobalContext:dropLastParentType(setter)
        local lastParentType = table.remove(parentsTypes)
        print("\nCONTEXT <- [" ..
            setter ..
            "] drop last parent type as [" ..
            lastParentType .. "] full chain after [\n" .. tableToString(parentsTypes) .. "\n]")
    end

    --- Method to add last child model name
    --- @param setter string # The name of the script who set the value.
    --- @param modelName string # last model name of child
    function GlobalContext:addLastChildrenModelName(setter, modelName)
        table.insert(childrenModelNames, modelName)
        print("\nCONTEXT <- [" ..
            setter ..
            "] add last model name as [" ..
            modelName .. "] full chain after [\n" .. tableToString(childrenModelNames) .. "\n]")
    end

    --- Method to get last child model name
    --- @param getter string # The name of the script who set the value.
    --- @return string? # last child model name
    function GlobalContext:getLastChildrenModelName(getter)
        local lastChildModelName = childrenModelNames[#childrenModelNames]
        print("\nCONTEXT -> [" ..
            getter ..
            "] get last model name as [" ..
            lastChildModelName .. "] full chain is [\n" .. tableToString(childrenModelNames) .. "\n]")
        return lastChildModelName
    end

    --- Method to drop last child model name
    --- @param setter string # The name of the script who set the value.
    function GlobalContext:dropLastChildrenModelName(setter)
        local lastChildModelName = table.remove(childrenModelNames)
        print("\nCONTEXT <- [" ..
            setter ..
            "] drop last children model name as [" ..
            lastChildModelName .. "] full chain is [\n" .. tableToString(childrenModelNames) .. "\n]")
    end

    --- Method to add include to final model
    --- @param setter string # The name of the script who set the value.
    --- @param modelName string # Model name, function can be called for other model when current is not finished.
    --- @param writeOperations WriteOperation[] # Collected by visitor write operation
    function GlobalContext:addIncludes(setter, modelName, writeOperations)
        if models[modelName] then
            models[modelName]:addIncludes(writeOperations)
        else
            local model = Model.new()
            model:addIncludes(writeOperations)
            models[modelName] = model
        end
        print("\nCONTEXT <- [" ..
            setter .. "] add include to model [" .. modelName .. "] as [\n" .. tableToString(writeOperations) .. "\n]")
    end

    --- Method to adapt include to new model
    --- @param setter string # The name of the script who set the value.
    --- @param modelName string # Model name, function can be called for other model when current is not finished.
    --- @param writeOperations WriteOperation[] # Collected by visitor write operation
    function GlobalContext:adaptIncludes(setter, modelName, writeOperations)
        local adaptedWriteOperations = adaptWriteOperations(writeOperations, modelName)
        if models[modelName] then
            models[modelName]:addIncludes(adaptedWriteOperations)
        else
            local model = Model.new()
            model:addIncludes(adaptedWriteOperations)
            models[modelName] = model
        end
        print("\nCONTEXT <- [" ..
            setter ..
            "] adapted include to model [" .. modelName .. "] as [\n" .. tableToString(adaptedWriteOperations) .. "\n]")
    end

    --- Method to get includes of final model
    --- @param getter string # The name of the script who get the value.
    --- @param modelName string # Model name, function can be called for other model when current is not finished.
    --- @return WriteOperation[] # Collected by visitor write operation
    function GlobalContext:getIncludes(getter, modelName)
        local includes = {}
        if models[modelName] then
            includes = models[modelName].includes
        end
        print("\nCONTEXT -> [" ..
            getter .. "] get includes of model [" .. modelName .. "] as [\n" .. tableToString(includes) .. "\n]")
        return includes
    end

    --- Method to add property to final model
    --- @param setter string # The name of the script who set the value.
    --- @param modelName string # Model name, function can be called for other model when current is not finished.
    --- @param writeOperations WriteOperation[] # Collected by visitor write operation
    function GlobalContext:addProperties(setter, modelName, writeOperations)
        if models[modelName] then
            models[modelName]:addProperties(writeOperations)
        else
            local model = Model.new()
            model:addProperties(writeOperations)
            models[modelName] = model
        end
        print("\nCONTEXT <- [" ..
            setter .. "] add property to model [" .. modelName .. "] as [\n" .. tableToString(writeOperations) .. "\n]")
    end

    --- Method to adapting property to model with replacing target file name
    --- @param setter string # The name of the script who set the value.
    --- @param modelName string # Model name, function can be called for other model when current is not finished.
    --- @param writeOperations WriteOperation[] # Collected by visitor write operation
    function GlobalContext:adaptProperties(setter, modelName, writeOperations)
        local adaptedWriteOperations = adaptWriteOperations(writeOperations, modelName)
        if models[modelName] then
            models[modelName]:addProperties(adaptedWriteOperations)
        else
            local model = Model.new()
            model:addProperties(adaptedWriteOperations)
            models[modelName] = model
        end
        print("\nCONTEXT <- [" ..
            setter ..
            "] adapt properties to model [" .. modelName .. "] as [\n" .. tableToString(adaptedWriteOperations) .. "\n]")
    end

    --- Method to get properties of model
    --- @param getter string # The name of the script who get the value.
    --- @param modelName string # Model name, function can be called for other model when current is not finished.
    --- @return WriteOperation[] # Collected by visitor write operation
    function GlobalContext:getProperties(getter, modelName)
        local properties = {}
        if models[modelName] then
            properties = models[modelName].properties
        end
        print("\nCONTEXT -> [" ..
            getter .. "] get properties of model [" .. modelName .. "] as [\n" .. tableToString(properties) .. "\n]")
        return properties
    end

    --- Method to add method to final model
    --- @param setter string # The name of the script who set the value.
    --- @param modelName string # Model name, function can be called for other model when current is not finished.
    --- @param writeOperations WriteOperation[] # Collected by visitor write operation
    function GlobalContext:addMethods(setter, modelName, writeOperations)
        if models[modelName] then
            models[modelName]:addMethod(writeOperations)
        else
            local model = Model.new()
            model:addMethod(writeOperations)
            models[modelName] = model
        end
        print("\nCONTEXT <- [" ..
            setter .. "] add method to model [" .. modelName .. "] as [\n" .. tableToString(writeOperations) .. "\n]")
    end

    --- Method to adapting method to new model name
    --- @param setter string # The name of the script who set the value.
    --- @param modelName string # Model name, function can be called for other model when current is not finished.
    --- @param writeOperations WriteOperation[] # Collected by visitor write operation
    function GlobalContext:adaptMethods(setter, modelName, writeOperations)
        local adaptedWriteOperations = adaptWriteOperations(writeOperations, modelName)
        if models[modelName] then
            models[modelName]:addMethod(adaptedWriteOperations)
        else
            local model = Model.new()
            model:addMethod(adaptedWriteOperations)
            models[modelName] = model
        end
        print("\nCONTEXT <- [" ..
            setter ..
            "] adapted method to model [" .. modelName .. "] as [\n" .. tableToString(adaptedWriteOperations) .. "\n]")
    end

    --- Method to get methods of final model
    --- @param getter string # The name of the script who get the value.
    --- @param modelName string # Model name, function can be called for other model when current is not finished.
    --- @return WriteOperation[] # Collected by visitor write operation
    function GlobalContext:getMethods(getter, modelName)
        local methods = {}
        if models[modelName] then
            methods = models[modelName].methods
        end
        print("\nCONTEXT -> [" ..
            getter .. "] get methods of model [" .. modelName .. "] as [\n" .. tableToString(methods) .. "\n]")
        return methods
    end

    --- Method to get collected model
    --- @param getter string # The name of the script who get the value
    --- @param modelName string # Required model name
    --- @return Model? # collected model
    function GlobalContext:getModel(getter, modelName)
        print("\nCONTEXT -> [" ..
            getter .. "] get model [" .. modelName .. "] as [\n" .. tableToString(models[modelName]) .. "\n]")
        return models[modelName]
    end

    --- For instance, if we have already constructed this model, we can discard the old variant for new
    --- processing. The translator attempts to reconstruct every reference from scratch.
    --- @param getter string # The name of the script who get the value
    --- @param modelName string # Required model name
    function GlobalContext:dropModel(getter, modelName)
        print("\nCONTEXT <- [" ..
            getter .. "] drop model [" .. modelName .. "] with content [\n" .. tableToString(models[modelName]) .. "\n]")
        models[modelName] = nil
    end

    return instance
end

--- global accessed context within scripts
global_context = GlobalContext:new()

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
    print("-----------------------")
end

--- Print table to console
--- @param t table # table for conversion
--- @param indent number|nil # level of incapsulation
function printTable(t, indent)
    indent = indent or 10

    if t == null then
        print(string.rep(" ", indent) .. "Argument is null!")
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
        local formatting = string.rep(" ", indent) .. "empty"
        print(formatting)
    else
        for key, value in pairs(t) do
            local formatting = string.rep(" ", indent) .. tostring(key) .. ": "

            if type(value) == "table" then
                print(formatting)
                printTable(value, indent + 4)
            else
                print(formatting .. tostring(value))
            end
        end
    end
end

--- Table into string
--- @param tbl table|nil # table for conversion
--- @param indent number|nil # level of incapsulation
--- @return string # string interpretation of table
function tableToString(tbl, indent)
    indent = indent or 10

    if tbl == null then
        return string.rep(" ", indent) .. "Argument is null!"
    end

    if tbl == nil then
        return string.rep(" ", indent) .. "Argument is nil!"
    end

    if type(tbl) ~= "table" then
        return string.rep(" ", indent) .. "Argument is not a table type with value [" .. tostring(tbl) .. "]"
    end

    local result = {}
    local spacing = string.rep(" ", indent)

    if isTableEmpty(tbl) then
        local formatting = spacing .. "empty"
        table.insert(result, formatting)
    else
        for key, value in pairs(tbl) do
            local formatting = spacing .. tostring(key) .. ": "
            if type(value) == "table" then
                table.insert(result, formatting)
                table.insert(result, tableToString(value, indent + 4))
            else
                table.insert(result, formatting .. tostring(value))
            end
        end
    end


    return table.concat(result, "\n")
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

--- Function decorator for logging
--- @param funcName string # name of called function
--- @param mainFunc function # main function with arguments from Rust code
--- @param beforeDecorator function? # decorator for calling before mainFunc with same args as mainFunc
--- @param afterDecorator function? # decorator for calling after mainFunc with same args as mainFunc
function functionCallAndLog(funcName, mainFunc, beforeDecorator, afterDecorator)
    return function(...)
        print("CALL <- [" .. funcName .. "]")

        local args = { ... }
        for i, v in ipairs(args) do
            if type(v) == "table" then
                print("    arg" .. i .. " = [table]")
                printTable(v, 8)
            else
                print("    arg" .. i .. " = " .. tostring(v))
            end
        end
        if beforeDecorator ~= nil then
            beforeDecorator(...)
        end
        local result = mainFunc(...)
        if afterDecorator ~= nil then
            afterDecorator(...)
        end
        if type(result) == "table" then
            print("    return = [table]")
            printTable(result, 8)
        else
            print("    return = " .. tostring(result))
        end
        return result
    end
end

--- Type of parent for children schemas
--- @class ParentType
--- @field ARRAY string # Parent for arrays items
--- @field OBJECT string # Parent for objects properties
--- @field ALL_OF string # Parent for allOf collections
--- @field ONE_OF string # Parent for oneOf collections
--- @field ANY_OF string # Parent for anyOf collections
--- @field NOT string # Parent for not property
--- @field ADDITIONAL string # Parent for additional properties
ParentType = {}

ParentType.ARRAY = "ARRAY"
ParentType.OBJECT = "OBJECT"
ParentType.ADDITIONAL = "ADDITIONAL"
ParentType.ALL_OF = "ALL_OF"
ParentType.ANY_OF = "ANY_OF"
ParentType.ONE_OF = "ONE_OF"
ParentType.NOT = "NOT"

--- For WriteOperation it is mode of operation on disk
--- @class WriteMode
--- @field APPEND string # Add something at end of file, if file does not exists create it
--- @field PREPEND string # Add something at start of file, if file does not exists create it
--- @field REMOVE string # Remove some model file if it exists or no op
WriteMode = {}

WriteMode.APPEND = "APPEND"
WriteMode.PREPEND = "PREPEND"
WriteMode.REMOVE = "REMOVE"

--- Script is an element of the visitor call chain
--- @class Script
--- @field PRELUDE string
--- @field RESPONSES_START string
--- @field RESPONSES_END string
--- @field RESPONSE_START string
--- @field RESPONSE_END string
--- @field RESPONSE_HEADERS_START string
--- @field RESPONSE_HEADERS_END string
--- @field RESPONSE_HEADER_START string
--- @field RESPONSE_HEADER_END string
--- @field RESPONSE_HEADER_EXAMPLE string
--- @field RESPONSE_HEADER_EXAMPLES_EXAMPLE string
--- @field RESPONSE_HEADER_EXAMPLES_START string
--- @field RESPONSE_HEADER_EXAMPLES_END string
--- @field RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_START string
--- @field RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_END string
--- @field RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_START string
--- @field RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_END string
--- @field RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_EXAMPLE string
--- @field RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_ENCODING_START string
--- @field RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_ENCODING_END string
--- @field SCHEMAS_START string
--- @field SCHEMAS_END string
--- @field SCHEMA_START string
--- @field SCHEMA_END string
--- @field SCHEMA_EXTERNAL_DOCS string
--- @field SCHEMA_EXAMPLE string
--- @field SCHEMA_DEFAULT string
--- @field SCHEMA_DISCRIMINATOR string
--- @field SPEC_EXTERNAL_DOCS string
--- @field SPEC_START string
--- @field SPEC_END string
--- @field SPEC_TAG_EXTERNAL_DOCS string
--- @field SPEC_TAG string
--- @field SPEC_TAGS_END string
--- @field SPEC_TAGS_START string
--- @field SPEC_SERVERS_START string
--- @field SPEC_SERVER string
--- @field SPEC_SERVER_VARIABLE string
--- @field SPEC_SERVERS_END string
--- @field SPEC_INFO string
--- @field SPEC_INFO_CONTACT string
--- @field SPEC_INFO_LICENSE string
--- @field SPEC_SECURITIES_START string
--- @field SPEC_SECURITY string
--- @field SPEC_SECURITIES_END string
--- @field OBJECT_START string
--- @field OBJECT_END string
--- @field ANY_SCHEMA string
--- @field NOT_PROPERTY_START string
--- @field NOT_PROPERTY_END string
--- @field OBJECT_ADDITIONAL_PROPERTIES_ANY string
--- @field OBJECT_ADDITIONAL_PROPERTIES_START string
--- @field OBJECT_ADDITIONAL_PROPERTIES_END string
--- @field STRING_PROPERTY string
--- @field NUMBER_PROPERTY string
--- @field INTEGER_PROPERTY string
--- @field ARRAY_PROPERTY_START string
--- @field ARRAY_PROPERTY_END string
--- @field BOOLEAN_PROPERTY string
--- @field ONE_OF_START string
--- @field ONE_OF_END string
--- @field ALL_OF_START string
--- @field ALL_OF_END string
--- @field ANY_OF_START string
--- @field ANY_OF_END string
Script = {}

Script.PRELUDE = "PRELUDE"
Script.RESPONSES_START = "RESPONSES_START"
Script.RESPONSES_END = "RESPONSES_END"
Script.RESPONSE_START = "RESPONSE_START"
Script.RESPONSE_END = "RESPONSE_END"
Script.RESPONSE_HEADERS_START = "RESPONSE_HEADERS_START"
Script.RESPONSE_HEADERS_END = "RESPONSE_HEADERS_END"
Script.RESPONSE_HEADER_START = "RESPONSE_HEADER_START"
Script.RESPONSE_HEADER_END = "RESPONSE_HEADER_END"
Script.RESPONSE_HEADER_EXAMPLE = "RESPONSE_HEADER_EXAMPLE"
Script.RESPONSE_HEADER_EXAMPLES_EXAMPLE = "RESPONSE_HEADER_EXAMPLES_EXAMPLE"
Script.RESPONSE_HEADER_EXAMPLES_START = "RESPONSE_HEADER_EXAMPLES_START"
Script.RESPONSE_HEADER_EXAMPLES_END = "RESPONSE_HEADER_EXAMPLES_END"
Script.RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_START = "RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_START"
Script.RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_END = "RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_END"
Script.RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_START =
"RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_START"
Script.RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_END = "RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_END"
Script.RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_EXAMPLE =
"RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_EXAMPLE"
Script.RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_ENCODING_START =
"RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_ENCODING_START"
Script.RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_ENCODING_END =
"RESPONSE_HEADER_EXAMPLES_EXAMPLE_FORMAT_MEDIA_TYPE_ENCODING_END"
Script.SCHEMAS_START = "SCHEMAS_START"
Script.SCHEMAS_END = "SCHEMAS_END"
Script.SCHEMA_START = "SCHEMA_START"
Script.SCHEMA_END = "SCHEMA_END"
Script.SCHEMA_EXTERNAL_DOCS = "SCHEMA_EXTERNAL_DOCS"
Script.SCHEMA_EXAMPLE = "SCHEMA_EXAMPLE"
Script.SCHEMA_DEFAULT = "SCHEMA_DEFAULT"
Script.SCHEMA_DISCRIMINATOR = "SCHEMA_DISCRIMINATOR"
Script.SPEC_EXTERNAL_DOCS = "SPEC_EXTERNAL_DOCS"
Script.SPEC_START = "SPEC_START"
Script.SPEC_END = "SPEC_END"
Script.SPEC_TAG_EXTERNAL_DOCS = "SPEC_TAG_EXTERNAL_DOCS"
Script.SPEC_TAG = "SPEC_TAG"
Script.SPEC_TAGS_END = "SPEC_TAGS_END"
Script.SPEC_TAGS_START = "SPEC_TAGS_START"
Script.SPEC_SERVERS_START = "SPEC_SERVERS_START"
Script.SPEC_SERVER = "SPEC_SERVER"
Script.SPEC_SERVER_VARIABLE = "SPEC_SERVER_VARIABLE"
Script.SPEC_SERVERS_END = "SPEC_SERVERS_END"
Script.SPEC_INFO = "SPEC_INFO"
Script.SPEC_INFO_CONTACT = "SPEC_INFO_CONTACT"
Script.SPEC_INFO_LICENSE = "SPEC_INFO_LICENSE"
Script.SPEC_SECURITIES_START = "SPEC_SECURITIES_START"
Script.SPEC_SECURITY = "SPEC_SECURITY"
Script.SPEC_SECURITIES_END = "SPEC_SECURITIES_END"
Script.OBJECT_START = "OBJECT_START"
Script.OBJECT_END = "OBJECT_END"
Script.ANY_SCHEMA = "ANY_SCHEMA"
Script.NOT_PROPERTY_START = "NOT_PROPERTY_START"
Script.NOT_PROPERTY_END = "NOT_PROPERTY_END"
Script.OBJECT_ADDITIONAL_PROPERTIES_ANY = "OBJECT_ADDITIONAL_PROPERTIES_ANY"
Script.OBJECT_ADDITIONAL_PROPERTIES_START = "OBJECT_ADDITIONAL_PROPERTIES_START"
Script.OBJECT_ADDITIONAL_PROPERTIES_END = "OBJECT_ADDITIONAL_PROPERTIES_END"
Script.STRING_PROPERTY = "STRING_PROPERTY"
Script.NUMBER_PROPERTY = "NUMBER_PROPERTY"
Script.INTEGER_PROPERTY = "INTEGER_PROPERTY"
Script.ARRAY_PROPERTY_START = "ARRAY_PROPERTY_START"
Script.ARRAY_PROPERTY_END = "ARRAY_PROPERTY_END"
Script.BOOLEAN_PROPERTY = "BOOLEAN_PROPERTY"
Script.ONE_OF_START = "ONE_OF_START"
Script.ONE_OF_END = "ONE_OF_END"
Script.ALL_OF_START = "ALL_OF_START"
Script.ALL_OF_END = "ALL_OF_END"
Script.ANY_OF_START = "ANY_OF_START"
Script.ANY_OF_END = "ANY_OF_END"

--- It is a special predefined global value similar to nil. However, it
--- specifically used for data passed from the translator (Rust code) that has a nil value.
--- userdata(nil) == null
--- @class null

--- container for possible model names
--- @class ModelName
--- @field base string # The base name from OpenAPI
--- @field extended string|null # The extended name from x-ot-model-name, if present or special null value.

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
    local instance = setmetatable({}, WriteOperation)
    instance.code = code
    instance.file = modelName .. ".java"
    instance.mode = WriteMode.PREPEND
    return instance
end

--- @param modelName string # output model name to construct file name
function WriteOperation.new_remove(modelName)
    local instance = setmetatable({}, WriteOperation)
    instance.code = nil
    instance.file = modelName .. ".java"
    instance.mode = WriteMode.REMOVE
    return instance
end

--- @param code string # produced code
--- @param modelName string # data for generate file name
function WriteOperation.new_append(code, modelName)
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

--- We gather elements to create a model for disk storage. A typical Java model consists of includes
--- and code block related to a model class, containing properties and methods. Since visitors might need
--- to add elements to the model at any time (when they called), they are represented as separate lists
--- of disk write operations. For example, before saving, we can sort our write operations based on
--- certain criteria if needed.
--- @class Model
--- @field includes WriteOperation[]
--- @field properties WriteOperation[]
--- @field methods WriteOperation[]
Model = {}
Model.__index = Model

function Model.new()
    local instance = setmetatable({}, Model)
    instance.includes = {}
    instance.properties = {}
    instance.methods = {}
    return instance
end

--- @param writeOperations WriteOperation[] # Collected by visitor write operation
function Model:addIncludes(writeOperations)
    self.includes = concatTables(self.includes, writeOperations)
end

--- @return WriteOperation[] # Collected by visitor write operation
function Model:getIncludes()
    return self.includes
end

--- @param writeOperations WriteOperation[] # Collected by visitor write operation
function Model:addProperties(writeOperations)
    self.properties = concatTables(self.properties, writeOperations)
end

--- @return WriteOperation[] # Collected by visitor write operation
function Model:getProperties()
    return self.properties
end

--- @param writeOperations WriteOperation # Collected by visitor write operation
function Model:addMethod(writeOperations)
    self.methods = concatTables(self.methods, writeOperations)
end

--- @return WriteOperation # Collected by visitor write operation
function Model:getMethods()
    return self.methods
end

--- Function to get the value of the model name from x-ot-name property of schema if it exists, or the
--- `base` field otherwise and use it as model name.
--- @param modelName ModelName
--- @return string # model name
local function getName(modelName)
    local extendedModelName = modelName.extended
    if extendedModelName == null then
        return modelName.base
    elseif type(extendedModelName) == "string" then -- just for compiler calm
        return extendedModelName
    else
        error("Extended model name is null")
    end
end

--- Function to concatenate strings from array except last N
--- @param namesStack ModelName[] # model names chain
--- @return string|nil # conctatenated model names except last N
local function concatenateExceptLastN(namesStack, n)
    local length = #namesStack
    if length - n <= 0 then
        return nil
    end

    local result = ""
    for i = 1, length - n do
        result = result .. getName(namesStack[i])
    end

    return result
end

--- Replaces placeholders in the string with corresponding values from a table.
--- Placeholders must be in the format ${key} in string.
--- @param str string # The string containing placeholders.
--- @param parameters table # A table containing key-value pairs for interpolation.
--- @return string # A new string where placeholders have been replaced by their corresponding values.
function interpolate(parameters, str)
    return (str:gsub("($%b{})", function(w) return parameters[w:sub(3, -2)] or w end))
end

--- Function to get generic parent model name
--- @param namesStack ModelName[]
--- @return string|nil # parent model name
function getParentModelName(namesStack)
    local parentModelName = namesStack[#namesStack - 1]
    if parentModelName == nil then
        return nil
    elseif parentModelName.extended == null then
        return concatenateExceptLastN(namesStack, 1)
    else
        local extendedModelName = parentModelName.extended
        if type(extendedModelName) == "string" then -- just for compiler calm
            return extendedModelName
        else
            error("Extended model name for parent is null")
        end
    end
end

--- Function to get generic Nth parent model name
--- @param namesStack ModelName[]
--- @param n integer # number of parent in stack
--- @return string # parent model name
function getNthParentModelNameMandatory(namesStack, n)
    local extendedModelName = namesStack[#namesStack - n].extended
    if extendedModelName == null or extendedModelName == nil then
        local parentModelName = concatenateExceptLastN(namesStack, n)
        if parentModelName == nil then
            error("Parent model name is null")
        else
            return parentModelName
        end
    elseif type(extendedModelName) == "string" then -- just for compiler calm
        return extendedModelName
    else
        error("Extended model name for parent is null")
    end
end

--- Function to get generic Nth parent model name
--- @param callStack string[]
--- @param n integer # number of parent in stack
--- @return string # parent model name
function getNthFromEndCallerScriptType(callStack, n)
    return callStack[#callStack - n]
end

--- Determines that there is a specified parent in the call chain
--- @param getter string # Caller function name
--- @param callStack string[] # An array of string constants.
--- @param allowedParents string[] # A list of constants to search for
--- @return boolean # if true then parent is found in call stack
function hasSpecifiedParentsInCallChain(getter, callStack, allowedParents)
    return findFirstMatchFromEnd(getter, callStack, allowedParents) ~= nil
end

--- Finds the first matching constant from the end of an array.
--- @param getter string # Caller function name
--- @param stringsArray string[] # An array of string constants.
--- @param searchList string[] # A list of constants to search for
--- @return string|nil # The first found constant from `searchList`, or `nil` if none is found.
function findFirstMatchFromEnd(getter, stringsArray, searchList)
    local searchSet = {}
    for _, const in ipairs(searchList) do
        searchSet[const] = true
    end

    for i = #stringsArray, 1, -1 do
        if searchSet[stringsArray[i]] then
            print("\nCALL -> [" ..
                getter ..
                "] get last string const as [" ..
                stringsArray[i] .. "] full strings array [\n" .. tableToString(stringsArray) .. "\n]")
            return stringsArray[i]
        end
    end

    print("\nCALL -> [" ..
        getter ..
        "] get last string const as [nil] full strings array [\n" .. tableToString(stringsArray) .. "\n]")

    return nil
end

--- Function to get generic current model name or error
--- @param namesStack ModelName[]
--- @return string # current model name or error
function getCurrentModelNameMandatory(namesStack)
    local extendedModelName = namesStack[#namesStack].extended
    if extendedModelName == null or extendedModelName == nil then
        local currentModelName = concatenateExceptLastN(namesStack, 0)
        if currentModelName == nil then
            error("Current model name not found")
        else
            return currentModelName
        end
    elseif type(extendedModelName) == "string" then -- just for compiler calm
        return extendedModelName
    else
        error("Extended model name for current model is null")
    end
end

--- Function to get current property name
--- @param namesStack ModelName[]
--- @return string # current property name
function getCurrentPropertyNameMandatory(namesStack)
    return getName(namesStack[#namesStack])
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

--- Returns a marker for property or model that it is required (as example @NonNull in java)
--- @param required boolean # Indicates if the property value (this object) is required
--- @param codeForMarker string # template for code for required marker
--- @return string # required marker text
function getRequiredMarker(required, codeForMarker)
    local requiredMarker = ""
    if required then
        requiredMarker = requiredMarker .. codeForMarker
    end
    return requiredMarker
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

--- Global variable containing parameters passed by the translator to the Lua code either from the OpenAPI
--- specification or from command line parameters.
--- Command line parameters take precedence over API specification parameters.
--- This construction is used solely to inform the Lua language server
--- about the existence of the global variable for convenience when writing scripts.
--- variable already set by Rust code
if false then
    ---@type any|null|nil # The type depends on how the parameters are specified in the command line or OpenAPI specification
    targetParameters = nil
end

--- This script is called first, at the beginning of all processing. It outputs the value of all parameters
--- passed to the script either from the OpenAPI specification or from the command line. Command line
--- parameters take precedence and override the specification parameters. Parameters are stored in the
--- global variable `targetParameters` created by the translator (Rust code) in the Lua context
function stub()
    printBreak()
    print("targetParamaters type: " .. type(targetParameters))
    print("targetParamaters value:")
    if type(targetParameters) == "table" then
        printTable(targetParameters)
    else
        print(targetParameters)
    end
    printBreak()
end

return stub
