--- This script is run after prelude.lua to set functions specific to a particular type of translation
--- That is, you can use a common set of visitors and prelude.lua to set general translation mechanisms
--- and target.lua to set specific translation mechanisms for, for example, translation from the OpenAPI 3
--- into a specific java target models

--- package.path concatenated with VISITORS_PATH and TARGET_PATH, modules can be placed to this paths
--- we can use any set of visitors in VISITORS_PATH

--- here real code for generating concrete Java output
CODE = require("code")

--- By leveraging the dynamic nature of Lua modules, we can replace visitors with any others,
--- allowing us to reuse some visitors from one set while incorporating parts from another.
--- As an example, one set of visitors is for model generation, and another set is for server
--- generation and client generation. The common set for model generation can be reused. Here
--- is a set of visitors and utility functions for generating.
VISITORS = require("cbase")

--- @param callId string? # some useful identifier of this visitor call
local function target(callId)
    print("Target script called")
end

--- During the target invocation, the prelude script has already been executed, so we can call functionCallAndLog.
--- However, during the script verification stage, they are called one by one just for checking, and this function
--- is not available there.
return functionCallAndLog("target", target)
