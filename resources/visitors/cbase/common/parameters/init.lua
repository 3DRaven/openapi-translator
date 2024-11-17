local M = {}
M.visitParametersStart = require("cbase.common.parameters.visitParametersStart")
M.visitParameterReference = require("cbase.common.parameters.visitParameterReference")
M.visitParametersEnd = require("cbase.common.parameters.visitParametersEnd")

M.cookie_parameter = require("cbase.common.parameters.cookie_parameter")
M.header_parameter = require("cbase.common.parameters.header_parameter")
M.path_parameter = require("cbase.common.parameters.path_parameter")
M.query_parameter = require("cbase.common.parameters.query_parameter")
return M
