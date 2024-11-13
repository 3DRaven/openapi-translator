--- This script is run after prelude.lua to set functions specific to a particular type of translation
--- That is, you can use a common set of visitors and prelude.lua to set general translation mechanisms
--- and target.lua to set specific translation mechanisms for, for example, translation from the OpenAPI 3
--- into a specific java target models

CODE = require("code")

--- @param callsStack Script[] # An array of Script objects representing the sequence of scripts executed in the visitor call chain
local function target(callsStack)
    print("Target script called")
end

--- During the target invocation, the prelude script has already been executed, so we can call functionCallAndLog.
--- However, during the script verification stage, they are called one by one just for checking, and this function
--- is not available there.
if functionCallAndLog == nil then
    return target
else
    return functionCallAndLog("target", target)
end
