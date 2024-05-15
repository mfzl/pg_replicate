use std::collections::HashMap;

use async_trait::async_trait;
use thiserror::Error;
use tokio_postgres::types::PgLsn;

use crate::{
    conversion::{TryFromReplicationMessage, TryFromTableRow},
    table::{ColumnSchema, TableId, TableName, TableSchema},
};

use self::postgres::{CdcStream, PostgresSourceError, TableCopyStream};

pub mod postgres;

#[derive(Debug, Error)]
pub enum SourceError {
    #[error("source error: {0}")]
    Postgres(#[from] PostgresSourceError),
}

#[async_trait]
pub trait Source<
    'a,
    'b,
    TE,
    TR: TryFromTableRow<TE> + Sync + Send,
    RE,
    RM: TryFromReplicationMessage<RE> + Sync + Send,
>
{
    fn get_table_schemas(&self) -> &HashMap<TableId, TableSchema>;

    async fn get_table_copy_stream(
        &self,
        table_name: &TableName,
        column_schemas: &'a [ColumnSchema],
        converter: &'a TR,
    ) -> Result<TableCopyStream<'a, TR, TE>, SourceError>;

    async fn get_cdc_stream(
        &'b self,
        start_lsn: PgLsn,
        converter: &'a RM,
    ) -> Result<CdcStream<'a, 'b, RM, RE>, SourceError>;
}