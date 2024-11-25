--- class contains custom code for properties
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
