local module = {}

--- @param codeBefore string
--- @param requiredMarker string
--- @param type string
--- @param name string
--- @return string #
function module.getPropertyCode(codeBefore, requiredMarker, type, name)
    return string.format("%s\n    private %s %s %s;\n", codeBefore or "", requiredMarker or "", type, name)
end

--- @return string #
function module.getRequiredImport()
    return "import javax.annotation.Nonnull;\n"
end

--- @return string #
function module.getRequiredMarker()
    return "@Nonnull"
end

--- @return string #
function module.getStringType()
    return "String"
end

--- @return string #
function module.getNumberType()
    return "Number"
end

--- @return string #
function module.getBooleanType()
    return "Boolean"
end

--- @return string #
function module.getIntegerType()
    return "Integer"
end

--- @param modelName string
--- @return string #
function module.getClassHeader(modelName)
    return string.format("\npublic class %s {\n\n", modelName)
end

--- @return string #
function module.getClassFooter()
    return "\n}\n"
end

--- @return string #
function module.getAnyType()
    return "Object"
end

--- @return string #
function module.getAdditionalPropertiesImport()
    return "import java.util.concurrent.ConcurrentHashMap;\n"
end

--- @return string #
function module.getArrayImport()
    return "import java.util.List;\n"
end

--- @param type string
--- @param propertyName string
--- @return string #
function module.getAdditionalPropertiesProperty(type, propertyName)
    return string.format("    private ConcurrentHashMap<String,%s> %s = new ConcurrentHashMap<>();\n",
        type, propertyName)
end

--- @param type string
--- @param propertyName string
--- @return string #
function module.getArrayProperty(type, propertyName)
    return string.format("    private  List<%s> %s = new List<>();\n",
        type, propertyName)
end

--- @param type string
--- @return string #
function module.getArrayAsType(type)
    return "List<" .. type .. ">"
end



--- @param arrayModelName string
--- @param childModelName string
--- @return string #
function module.getArrayAsModel(arrayModelName, childModelName)
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

return module