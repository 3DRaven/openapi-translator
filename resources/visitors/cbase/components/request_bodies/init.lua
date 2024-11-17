local M = {}
M.visitRequestBodyStart = require("cbase.components.request_bodies.visitRequestBodyStart")
M.visitRequestBodiesStart = require("cbase.components.request_bodies.visitRequestBodiesStart")
M.visitRequestBodyEnd = require("cbase.components.request_bodies.visitRequestBodyEnd")
M.visitRequestBodiesEnd = require("cbase.components.request_bodies.visitRequestBodiesEnd")
M.visitRequestBodyReferenceStart = require("cbase.components.request_bodies.visitRequestBodyReferenceStart")
M.visitRequestBodyReferenceEnd = require("cbase.components.request_bodies.visitRequestBodyReferenceEnd")

return M
