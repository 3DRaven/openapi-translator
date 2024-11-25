-- The set of functions in the module wasn't used because working with classes makes autocompletion
-- more convenient.
-- https://github.com/LuaLS/lua-language-server/issues/392
-- @diagnostic disable-next-line: luadoc-miss-module-name
-- @module default

--- We gather elements to create a model for disk storage. A typical Java model consists of includes
--- and code block related to a model class, containing properties and methods. Since visitors might need
--- to add elements to the model at any time (when they called), they are represented as separate lists
--- of disk write operations. For example, before saving, we can sort our write operations based on
--- certain criteria if needed.
--- @class TransactionalCodeBase:CodeBase
TransactionalCodeBase = setmetatable({}, { __index = CodeBase })
TransactionalCodeBase.__index = TransactionalCodeBase

--- @return TransactionalCodeBase
function TransactionalCodeBase.new()
    local instance = setmetatable(CodeBase.new(), TransactionalCodeBase)
    ---@type TransactionalCodeBase
    return instance
end

--- @return string #
function TransactionalCodeBase:getCustomImports()
    return "import org.springframework.transaction.annotation.Transactional;\n"
end

--- @return string #
function TransactionalCodeBase:getCustomMarkers()
    return "    @Transactional\n"
end

return TransactionalCodeBase
