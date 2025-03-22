--- The class contains a basic representation of the generated code and can have subclasses to refine the
--- implementation of specific elements, such as `TransactionalCodeBase`
--- @class CodeBase
CodeBase = {}
CodeBase.__index = CodeBase

function CodeBase.new()
    local instance = setmetatable({}, CodeBase)
    return instance
end

--- @param codeBefore string?
--- @param requiredMarker string?
--- @param type string
--- @param name string
--- @return string #
function CodeBase:getPropertyCode(codeBefore, requiredMarker, type, name)
    return string.format("%s    private %s %s %s;\n", codeBefore or "", requiredMarker or "", type, name)
end

--- @return string #
function CodeBase:getRequiredImport()
    return "import javax.annotation.Nonnull;\n"
end

--- @return string #
function CodeBase:getRequiredMarker()
    return "@Nonnull"
end

--- @return string? #
function CodeBase:getCustomImports()
    return nil
end

--- @return string? #
function CodeBase:getCustomMarkers()
    return nil
end

--- @return string #
function CodeBase:getStringType()
    return "String"
end

--- @return string #
function CodeBase:getNumberType()
    return "Number"
end

--- @return string #
function CodeBase:getBooleanType()
    return "Boolean"
end

--- @return string #
function CodeBase:getIntegerType()
    return "Integer"
end

--- @param modelName string
--- @return string #
function CodeBase:getClassHeader(modelName)
    return string.format("\npublic class %s {\n\n", modelName)
end

--- @return string #
function CodeBase:getClassFooter()
    return "\n}\n"
end

--- @return string #
function CodeBase:getAnyType()
    return "Object"
end

--- @return string #
function CodeBase:getAdditionalPropertiesImport()
    return "import java.util.concurrent.ConcurrentHashMap;\n"
end

--- @return string #
function CodeBase:getArrayImport()
    return "import java.util.List;\n"
end

--- @param type string
--- @param propertyName string
--- @return string #
function CodeBase:getAdditionalPropertiesProperty(type, propertyName)
    return string.format("    private ConcurrentHashMap<String,%s> %s = new ConcurrentHashMap<>();\n",
        type, propertyName)
end

--- @param type string
--- @param propertyName string
--- @return string #
function CodeBase:getArrayProperty(type, propertyName)
    return string.format("    private  List<%s> %s = new List<>();\n",
        type, propertyName)
end

--- @param type string
--- @return string #
function CodeBase:getArrayAsType(type)
    return "List<" .. type .. ">"
end

--- @param arrayModelName string
--- @param childModelName string
--- @return string #
function CodeBase:getArrayAsModel(arrayModelName, childModelName)
    local parameters = { className = arrayModelName, childClassName = childModelName }
    return interpolate(parameters, formatAndTrimIndent([[
        import java.util.List;

        public class ${className} {
            private List<${childClassName}> items;
            public ${className}() {}
            public ${className}(List<${childClassName}> items) {
                this.items = items;
            }
            public List<${childClassName}> get${className}() {
                return items;
            }
            public void set${className}(List<${childClassName}> items) {
                this.items = items;
            }
        }
        ]]))
end

--- @param className string # name of the model class
--- @return string # extension for class file (e.g. `.java`)
function CodeBase:getClassFileName(className)
    return className .. ".java"
end

return CodeBase
