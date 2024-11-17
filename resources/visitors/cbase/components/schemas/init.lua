local M = {}
M.visitSchemaEnd = require("cbase.components.schemas.visitSchemaEnd")
M.visitSchemaStart = require("cbase.components.schemas.visitSchemaStart")
M.visitSchemasEnd = require("cbase.components.schemas.visitSchemasEnd")
M.visitDiscriminator = require("cbase.components.schemas.visitDiscriminator")
M.visitSchemasStart = require("cbase.components.schemas.visitSchemasStart")
M.visitSchemaReferenceStart = require("cbase.components.schemas.visitSchemaReferenceStart")
M.visitSchemaReferenceEnd = require("cbase.components.schemas.visitSchemaReferenceEnd")
M.visitDefault = require("cbase.components.schemas.visitDefault")

M.kind = require("cbase.components.schemas.kind")
return M
