--- This script is run after prelude.lua to set functions specific to a particular type of translation
--- That is, you can use a common set of visitors and prelude.lua to set general translation mechanisms
--- and target.lua to set specific translation mechanisms for, for example, translation from the OpenAPI 3
--- into a specific java target models

function stub()
    printBreak()
    print("Target script called")
    printBreak()
end

return stub
