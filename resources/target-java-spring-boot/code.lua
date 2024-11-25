local module = {}

--- function return code variant with required code generation
--- @param variant string?
--- @return CodeBase # Class with methods for text generation with required variant or default
function module.getVariant(variant)
    --- @type CodeBase
    local defaultCodeBase = require("variants.default")
    if variant == nil then
        return defaultCodeBase.new()
    else
        --- @type CodeBase
        local customCodeBase = require("variants." .. variant)
        return customCodeBase.new()
    end
end

return module
