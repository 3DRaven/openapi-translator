--- This script is run after prelude.lua to set functions specific to a particular type of translation
--- That is, you can use a common set of visitors and prelude.lua to set general translation mechanisms
--- and target.lua to set specific translation mechanisms for, for example, translation from the OpenAPI 3
--- into a specific java target models

--- @return string #
function getPropertyCode(codeBefore, requiredMarker, type, name)
    return string.format("%s\n    private %s %s %s;\n", codeBefore or "", requiredMarker or "", type, name)
end

--- @return string #
function getRequiredImport()
    return "import javax.annotation.Nonnull;\n"
end

--- @return string #
function getRequiredMarker()
    return "@Nonnull"
end

function stub()
    printBreak()
    print("Target script called")
    printBreak()
end

return stub
