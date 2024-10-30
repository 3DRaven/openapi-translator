# Openapi-translator

## Introduction

Translator from OpenAPI v3 to some code. The core is written in Rust, and a set of visitors written in Lua is called for code generation. The overall goal of the project is to simplify the customization of the translator as much as possible.

```text
OpenAPI v3 translator

Usage: openapi-translator [OPTIONS] --prelude <PRELUDE_PATH> --visitors <VISITORS_PATH> <COMMAND>

Commands:
  test
  translate
  help       Print this message or the help of the given subcommand(s)

Options:
  -p, --parameters <PARAMETERS>   Parameters for target Lua scripts are simply JSON of arbitrary structure, which will be converted into a Lua table and passed to the scripts as a global parameter named targetParameters. These parameters will replace the parameters passed in the OpenAPI spec as x-ot-target-parameters
  -d, --prelude <PRELUDE_PATH>    Since visitors can be reused, the prelude dir contains in a separate script that runs at the start of the translation, where functions and modules that will be used in the general set of visitors to implement specific types of translation can be defined
  -s, --visitors <VISITORS_PATH>  The base directory for all visitors scripts, since for many types of translators, the final result only differs in specific small elements but is structurally similar, a common set of visitors can be used for different translation purposes
  -h, --help                      Print help
  -V, --version                   Print version
  ```

1. It is based on a set of lua scripts - visitors; in order to customize the generation, you do not need to rebuild the project.
2. Lua scripts work in one common context as a set of visitors, just like in a regular parser
3. By adding Lua Language server to vscode you will receive autocompletion and hints, working refactoring tools
4. The example is written to generate models for java, but it easy to add any logic
5. Supported references in OpenAPI spec, so you can split giant specs to parts

## How it Works

1. The openapi-translator utility is called.
2. It loads the OpenAPI 3 specification and parses it.
3. Then, traversal of the parsed model by visitors begins.
4. A prelude.lua (prelude dir parameter) script is executed before all visitors, where common functions can be written.
5. During the traversal, visitors are invoked within a shared Lua context, which forms the result of the translation.

In principle, the content of the visitors can be anything, but as an example, I made a translation from OpenAPI 3 to Java models.

## Scripts

1. All scripts can be modified without rebuilding the project.
2. In the tests parameter folder, there are "tests" These consist of a set of specifications (dir openapi), files resulting from the translation (actual), and reference files for comparison (expected). After the test generates a set of files, a `git diff` comparison is performed, and if there is a difference, the test fails. Thus, to test the scripts, it is not necessary to rebuild the project.

## Script example

```lua
--- This visitor is invoked when a property of type string is found.
--- Returns a string based on the provided string descriptor.
--- @param stringDescriptor StringType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
--- @return WriteOperation[] # Returns the output code and  file name for writing code
function visitStringProperty(stringDescriptor, extensions, callsStack)
    return addGenericPropertyCode(global_context.models:element(), "String", extensions)
end

return functionCallAndLog("visitStringProperty", visitStringProperty)
```

Lua Doc allows the Lua Language Server to understand types and suggest errors. Each script is called with a signature generally similar to the following:

`function visitStringProperty(stringDescriptor, extensions, callsStack)`

- `stringDescriptor`: This is just a set of data that the visitor must process from the specification.
- `extensions`: Extensions are `x-properties` that can be added to the specification for its extension, such as `x-ot-model-name`, an extension I've used to simplify the assignment of model names; in general, they can be anything.
- `callsStack`: Just stack off called visitors before
- `return value`: for all scripts it is WriteOperation[] it is single write operation to some file or file removing operation.

Every visitor always receives all associated information in full. Visitors can form a context by passing information to other visitors, for example, using:
`global_context:some_method()` or just by creating global values in same lua context

## Run example

`openapi-translator -d resources/java-spring-boot -s resources/generic-visitors -p '{"replaces":1}' test -n simple-model -t resources/java-spring-boot/tests`

it is test run for translate openapi spec in tests dir `simple-model` to actual models in dir `simple-model\actual`

## Logs

Every visitor call logged as `CALL <- [visitSchemaEnd]` with full list of parameters and `RETURN <- [visitSchemaEnd]`
return value. Every access to context logged as `CONTEXT ->`. Arrow `->` it is read and write to CALL, CONTEXT, RETURN targets.

```text
CALL <- [visitSchemaEnd]
    arg1 = simple_array
    arg2 = [table]
        x-ot-property-name: simpleArray
    arg3 = [table]
        x-ot-property-name: simpleArray
    arg4 = [table]
        1: prelude
        2: visitSpecStart
        3: visitComponentsStart
        4: visitSchemasStart
        5: visitSchemaStart
        6: visitObjectStart
        7: visitObjectPropertiesStart
        8: visitObjectPropertyStart
        9: visitSchemaStart
CONTEXT -> pop from stack, before
[
          stackName: modelsNames
          items:
              1: Response
              2: simple_array
]

RETURN <- [visitSchemaEnd] [table]
        empty
```

## Alternatives

- Hard to customize <https://github.com/OpenAPITools/openapi-generator>
- Only for Rust and clients generation <https://docs.rs/openapitor/latest/openapitor>
