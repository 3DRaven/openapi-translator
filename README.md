# openapi-translator
Translator from OpenAPI v3 to some code
```
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
5. Project under development


# How it Works

1. The openapi-translator utility is called.
2. It loads the OpenAPI 3 specification and parses it.
3. Then, traversal of the parsed model by visitors begins.
4. A prelude.lua script is executed before all visitors, where common functions can be written.
5. During the traversal, visitors are invoked within a shared Lua context, which forms the result of the translation.

In principle, the content of the visitors can be anything, but as an example, I made a translation from OpenAPI 3 to Java models.