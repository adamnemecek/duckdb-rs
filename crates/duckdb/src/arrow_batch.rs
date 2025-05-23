use super::{
    arrow::{datatypes::SchemaRef, record_batch::RecordBatch},
    Statement,
};

/// A handle for the resulting RecordBatch of a query.
#[must_use = "Arrow is lazy and will do nothing unless consumed"]
pub struct Arrow<'stmt> {
    pub(crate) stmt: Option<&'stmt Statement<'stmt>>,
}

#[allow(clippy::needless_lifetimes)]
impl<'stmt> Arrow<'stmt> {
    #[inline]
    pub(crate) fn new(stmt: &'stmt Statement<'stmt>) -> Self {
        Arrow { stmt: Some(stmt) }
    }

    /// return arrow schema
    #[inline]
    pub fn get_schema(&self) -> SchemaRef {
        self.stmt.unwrap().stmt.schema()
    }
}

#[allow(clippy::needless_lifetimes)]
impl<'stmt> Iterator for Arrow<'stmt> {
    type Item = RecordBatch;

    fn next(&mut self) -> Option<Self::Item> {
        Some(RecordBatch::from(&self.stmt?.step()?))
    }
}

/// A handle for the resulting RecordBatch of a query in streaming
#[must_use = "Arrow stream is lazy and will not fetch data unless consumed"]
#[allow(clippy::needless_lifetimes)]
pub struct ArrowStream<'stmt> {
    pub(crate) stmt: Option<&'stmt Statement<'stmt>>,
    pub(crate) schema: SchemaRef,
}

#[allow(clippy::needless_lifetimes)]
impl<'stmt> ArrowStream<'stmt> {
    #[inline]
    pub(crate) fn new(stmt: &'stmt Statement<'stmt>, schema: SchemaRef) -> Self {
        ArrowStream {
            stmt: Some(stmt),
            schema,
        }
    }

    /// return arrow schema
    #[inline]
    pub fn get_schema(&self) -> SchemaRef {
        self.schema.clone()
    }
}

#[allow(clippy::needless_lifetimes)]
impl<'stmt> Iterator for ArrowStream<'stmt> {
    type Item = RecordBatch;

    fn next(&mut self) -> Option<Self::Item> {
        Some(RecordBatch::from(&self.stmt?.stream_step(self.get_schema())?))
    }
}
