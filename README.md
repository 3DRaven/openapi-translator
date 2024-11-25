# Openapi-translator

## Introduction

Translator from OpenAPI v3 to some code. The core is written in Rust, and a set of visitors written in Lua is called for code generation. The overall goal of the project is to simplify the customization of the translator as much as possible.

```text
OpenAPI v3 translator

Usage: openapi-translator [OPTIONS] --target-scripts <TARGET_SCRIPTS_PATH> --visitors-scripts <VISITORS_SCRIPTS_PATH> <COMMAND>

Commands:
  test
  translate
  help       Print this message or the help of the given subcommand(s)

Options:
  -p, --target-parameters <PARAMETERS_JSON>
          Parameters for target Lua scripts are simply JSON of arbitrary structure, which will be converted into a Lua table and passed to the scripts as a global parameter named targetParameters. These parameters will replace the parameters passed in the OpenAPI spec as x-ot-target-parameters
  -a, --target-scripts <TARGET_SCRIPTS_PATH>
          Since visitors can be reused, the target dir contains in a separate script that runs at the start of the translation, where functions and modules that will be used in the general set of visitors to implement specific types of translation can be defined
  -i, --visitors-scripts <VISITORS_SCRIPTS_PATH>
          The base directory for all visitors scripts, since for many types of translators, the final result only differs in specific small elements but is structurally similar, a common set of visitors can be used for different translation purposes
  -h, --help
          Print help
  -V, --version
          Print version
```

1. It is based on a set of lua scripts - visitors; in order to customize the generation, you do not need to rebuild the project.
2. Lua scripts work in one common context as a set of visitors, just like in a regular parser
3. By adding Lua Language server to vscode you will receive autocompletion and hints, working refactoring tools
4. The example is written to generate models for java, but it easy to add any logic and reuse logic from example
5. Supported references in OpenAPI spec, so you can split giant specs to parts

## How it Works

1. The `openapi-translator` utility is invoked.
2. It loads and parses the OpenAPI 3 specification.
3. The `target.lua` script (located in the target-scripts directory) runs next, providing common functionality specific to the translation target. This script load prelude script in example, but logic can be any.
4. Following this, the parsed model undergoes traversal by visitors.
5. During this traversal, visitors operate within a shared Lua context, collaboratively producing the translation outcome.

In principle, the content of the visitors can be anything, but as an example, created a translation from OpenAPI 3 to Java models.

## Scripts

1. All scripts can be modified without rebuilding the project.
2. In the target-scripts folder, there are "tests" These consist of a set of specifications (dir openapi), files resulting from the translation (actual), and reference files for comparison (expected). After the test generates a set of files, a `diff` comparison is performed, and if there is a difference, the test fails. Thus, to test the scripts, it is not necessary to rebuild the project.

### Example

All scripts are placed in modules for convenient access and to perform initialization if needed (init.lua files)
Lua Doc allows the Lua Language Server to understand types and suggest errors. Each script is called with a signature generally similar to the following:

```lua
--- This visitor is invoked when a property of type string is found.
--- Returns a string based on the provided string descriptor.
--- @param stringDescriptor StringType # object descriptor
--- @param extensions table # table with free form with "x-" OpenAPI extensions for this level of spec
--- @param callId string? # some useful identifier of this visitor call
--- @return WriteOperation[] # Returns the output code and  file name for writing code
local function visitStringProperty(stringDescriptor, extensions, callId)
    local codeVariant = CODE.getVariant(extensions[Extensions.VARIANT])
    return STRUCT.addGenericPropertyCode(GLOBAL_CONTEXT.models:peek(), codeVariant:getStringType(), extensions)
end

return functionCallAndLog("visitStringProperty", visitStringProperty)
```

Often, we need to specify special conditions during generation in a specific part of the specification, such as marking the code, adding comments, or changing it completely. To implement this, code variants can be added. The function `CODE.getVariant(extensions[Extensions.VARIANT])` allows you to select a variant of the generated code from the modules in `target-scripts/variants`. For example, there's a variant for generating a `String` with an additional `@Transactional` annotation (in `resources/target-java-spring-boot/variants`). When the visitor encounters `x-ot-variant: transactional` in the specification, the corresponding replacement code will be triggered, returning the modified generated text. This way, it is easy to modify the generated code, creating multiple variants without cluttering the specification with unnecessary data.

### Visitors signature

`function visitStringProperty(stringDescriptor, extensions, callId)`

- `stringDescriptor`: This is just a set of data that the visitor must process from the specification.
- `extensions`: Extensions are `x-properties` that can be added to the specification for its extension, such as `x-ot-model-name`, an extension I've used to simplify the assignment of model names; in general, they can be anything.
- `callId`: Just some text id to logging visitor call and debug.
- `return value`: for all scripts it is WriteOperation[] it is write operations to some file or file removing operation.

Every visitor always receives all associated information in full. Visitors can form a context by passing information to other visitors, for example, using:
`GLOBAL_CONTEXT` or just by creating global values in same lua context

## Run example

```bash
openapi-translator --target-scripts resources/target-java-spring-boot --visitors-scripts resources/visitors
-p {\"replaces\":1} test -n simple-model -t resources/target-java-spring-boot/tests
```

it is test run for translate openapi spec in tests dir `simple-model` to actual models in dir `simple-model\actual`. `-p` used just for example, this parameter can pass some additional parameters 
to scripts

## Logs

Every visitor call logged as `CALL <- [visitSchemaEnd]` with full list of parameters and `RETURN <- [visitSchemaEnd]` return value. Every access to context logged as `CONTEXT ->`. Arrow `->` it is read and write to CALL, CONTEXT, RETURN targets.

```text
# link-22
CALL <- [visitAnySchemaStart]
    arg1 = [table]
        allOf:
          1:
            type: object
            properties:
              items:
                items:
                  $ref: #/components/schemas/CustomCode
                type: array
              security:
                x-ot-variant: transactional
                type: string
        type: object
    arg2 = [table] empty
    arg3 = userdata: (nil)
    Found unknown combination of properties in OpenAPI 3 spec,
    will be used separated sets of names for every known parts
CONTEXT <- push to stack [models], after
[
stackName: models
items:
  1:
    includes:
      stackName: any-schema->includes
      items: empty
    methods:
      stackName: any-schema->methods
      items: empty
    required: empty
    name: any-schema
    properties:
      stackName: any-schema->properties
      items: empty
]

RETURN <- [visitAnySchemaStart] [table] empty
```

After any error will be printed visitors calls stack

```text
The call stack, markdown links (#link-x) work and are clickable:
    1: [0](#link-0) prelude -> {no-id} : empty
    2: [1](#link-1) target -> {no-id}
    3: [2](#link-2) visitSpecStart -> {resources/target-java-spring-boot/tests/simple-model/openapi/openapi.yml}
```

Every visitor call marked by Markdown links (in example it is # link-22), so if log opened as Markdown
file, this links `[22](#link-22)` are clickable and can be used to navigation in log file.

When some model name found first time it be added to call description
`    25: [24](#link-24) visitSchemaStart -> {no-id} : SomeModelName`, after model processed it be dropped
`    66: [65](#link-65) visitMediaTypeEnd -> {application/json} : empty` or replaced to new found model name

## VSCode

By configuring `"lldb.launch.stdio": [null, "openapi-translator.log", null]`, logs are separated: LUA logs are saved to the `openapi-translator.log` file, while Rust logs appear in the console. This setup works in debug mode. Additionally, by setting `"files.associations": {"openapi-translator.log": "markdown"}`, the log file will open as a Markdown document every time.

## Alternatives

- Hard to customize <https://github.com/OpenAPITools/openapi-generator>
- Only for Rust and clients generation <https://docs.rs/openapitor/latest/openapitor>
