--- This script is called first, at the beginning of all processing. It outputs the value of all parameters
--- passed to the script either from the OpenAPI specification or from the command line. Command line
--- parameters take precedence and override the specification parameters. Parameters are stored in the
--- global variable `targetParameters` created by the translator (Rust code) in the Lua context
function stub()
    printBreak()
    print("Target script called")
    printBreak()
end

return stub
