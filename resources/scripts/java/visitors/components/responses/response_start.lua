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

--- This visitor is invoked before processing response
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param response Response # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitResponseStart(namesStack, response, extensions, callsStack)
    return {}
end

return functionCallAndLog("visitResponseStart", visitResponseStart)
