local M = {}

local prelude = require("cbase.prelude")
prelude(NULL)

M.visitSpecExternalDocs = require("cbase.visitSpecExternalDocs")
M.visitSpecEnd = require("cbase.visitSpecEnd")
M.visitSpecStart = require("cbase.visitSpecStart")

M.struct = require("cbase.struct")
M.common = require("cbase.common")
M.components = require("cbase.components")
M.info = require("cbase.info")
M.paths = require("cbase.paths")
M.tags = require("cbase.tags")
return M