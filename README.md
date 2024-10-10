
# openapi-translator

Translator from OpenAPI v3 to some code. Project under development.

```text
Usage: openapi-translator [OPTIONS] --target <TARGET> <COMMAND>

Commands:
  test       
  translate  
  help       Print this message or the help of the given subcommand(s)

Options:
  -t, --target <TARGET>
          Name of dir with translation scripts and tests
  -p, --target-parameters <TARGET_PARAMETERS>
          Parameters for target Lua scripts are simply JSON of arbitrary structure, which will be converted into a Lua table and passed to the scripts as a global parameter named targetParameters. These parameters will replace the parameters passed in the OpenAPI spec as x-ot-target-parameters
  -r, --resources <RESOURCES>
          Base dir for all translator resources [default: resources]
  -h, --help
          Print help
  -V, --version
          Print version
```

1. It is based on a set of lua scripts; in order to customize the generation, you do not need to rebuild the project.
2. Lua scripts work in one common context as a set of visitors, just like in a regular parser
3. By adding Lua Language server to vscode you will receive autocompletion and hints, working refactoring tools
4. The example is written to generate models for java
5. Supported references in OpenAPI spec, so you can split giant specs to parts

# How it Works

1. The openapi-translator utility is called.
2. It loads the OpenAPI 3 specification and parses it.
3. Then, traversal of the parsed model by visitors begins.
4. A prelude.lua script is executed before all visitors, where common functions can be written.
5. During the traversal, visitors are invoked within a shared Lua context, which forms the result of the translation.

In principle, the content of the visitors can be anything, but as an example, I made a translation from OpenAPI 3 to Java models.

# Scripts

1. All scripts are located in the resources folder.
2. They can be modified without rebuilding the project.
3. In the same folder, there are "tests" These consist of a set of specifications (dir openapi), files resulting from the translation (actual), and reference files for comparison (expected). After the test generates a set of files, a `git diff` comparison is performed, and if there is a difference, the test fails. Thus, to test the scripts, it is not necessary to rebuild the project.

The overall goal of the project is to simplify the customization of the translator as much as possible.

## Script example

```lua
--- Represents property of type string.
---@class StringDescriptor
---@field format string | nil # The format of the string type
---@field pattern string | nil                 # The pattern for the string type
---@field enum string[] | nil                  # The enumeration of possible string values
---@field min_length integer | nil              # The minimum length of the string
---@field max_length integer | nil              # The maximum length of the string

--- This visitor is invoked when a property of type string is found.
--- Returns a string based on the provided string descriptor.
--- @param namesStack ModelName[] # chain of model names from root to this point
--- @param stringDescriptor StringDescriptor # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitStringProperty(namesStack, stringDescriptor, extensions)
    local parentModelName = getParentModelName(namesStack)
    if parentModelName == nil then
        --- This is possible if a schema is constructed as a separate value;
        --- such a schema is not a model but can be used as a reference in other schemas.
        --- For example, you can add descriptions and other elements to it instead of copying them
        --- within the specification. So, we simply don't need to do anything in this case.
        print("String property without parent skip")
    else
        local parentType = global_context:getLastParentType("visitStringProperty")
        if parentType == ParentType.OBJECT or
            parentType == ParentType.ALL_OF then
            local currentPropertyName = getCurrentPropertyNameMandatory(namesStack)
            local required = global_context:isPropertyRequired("visitStringProperty", parentModelName,
                currentPropertyName)
            local requiredMarker = getRequiredMarker(required, "@NonNull ")

            local code = string.format("    private %sString %s;\n", requiredMarker,
                currentPropertyName);

            global_context:addProperties("visitStringProperty", parentModelName,
                { WriteOperation.new_append(code, parentModelName) })
        elseif parentType == ParentType.ARRAY then
        elseif parentType == ParentType.ADDITIONAL then
        else
            error("Unknown parent type for String")
        end
    end
    return {}
end

local function beforeDecorator()
    global_context:addLastChildrenModelName("visitStringProperty", "String")
end

return functionCallAndLog("visitStringProperty", visitStringProperty, beforeDecorator)

```

Lua Doc allows the Lua Language Server to understand types and suggest errors. Each script is called with a signature generally similar to the following:

`function visitStringProperty(namesStack, stringDescriptor, extensions)`

- `namesStack`: This is simply the path to this point in the specification, containing the names of models and properties. Visitors can be implemented without this, but I found it more convenient; however, it is optional to use.

- `stringDescriptor`: This is just a set of data that the visitor must process from the specification.

- `extensions`: Extensions are `x-properties` that can be added to the specification for its extension, such as `x-ot-name`, an extension I've used to simplify the assignment of model names; in general, they can be anything.
- `return value`: for all scripts it is WriteOperation[] it is single write operation to some file or file removing operation.

Every visitor always receives all associated information in full. Visitors can form a context by passing information to other visitors, for example, using:
`global_context:addLastChildrenModelName("visitStringProperty", "String")`
For the "parent" visitor, you can convey that a string type is being processed.

# run example

`openapi-translator -r ./resources -t java translate --spec ./openapi.yml --out ./`
