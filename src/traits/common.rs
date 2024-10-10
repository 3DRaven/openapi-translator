use openapiv3::Schema;

pub trait AsSchemaRef {
    fn as_schema(&self) -> &Schema;
}

impl AsSchemaRef for Schema {
    fn as_schema(&self) -> &Schema {
        self
    }
}

impl AsSchemaRef for Box<Schema> {
    fn as_schema(&self) -> &Schema {
        self.as_ref()
    }
}
