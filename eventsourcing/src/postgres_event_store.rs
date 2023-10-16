use crate::aggregate::Aggregate;
use crate::event_envelop::EventEnvelope;
use crate::event_store::EventStore;
use crate::serialized_event::SerializedEvent;
use crate::sql_query_factory::SqlQueryFactory;
use anyhow::Result;
use async_trait::async_trait;
use futures::TryStreamExt;
use serde_json::Value;
use sqlx::postgres::PgRow;
use sqlx::{Pool, Postgres, Row};
use std::collections::HashMap;

const DEFAULT_EVENT_TABLE: &str = "events";
const DEFAULT_STREAMING_CHANNEL_SIZE: usize = 200;

pub struct PostgresEventStore {
    pool: Pool<Postgres>,
    query_factory: SqlQueryFactory,
}

impl PostgresEventStore {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            pool,
            query_factory: SqlQueryFactory::new(DEFAULT_EVENT_TABLE),
        }
    }
}

impl PostgresEventStore {
    async fn select_events<A: Aggregate>(
        &self,
        aggregate_id: &str,
        query: &str,
    ) -> Result<Vec<SerializedEvent>> {
        let mut rows = sqlx::query(query)
            .bind(A::aggregate_type())
            .bind(aggregate_id)
            .fetch(&self.pool);
        let mut result: Vec<SerializedEvent> = Default::default();
        while let Some(row) = rows.try_next().await? {
            result.push(PostgresEventStore::deserialize_event(row)?);
        }
        Ok(result)
    }

    fn deserialize_event(row: PgRow) -> Result<SerializedEvent> {
        let aggregate_type: String = row.get("aggregate_type");
        let aggregate_id: String = row.get("aggregate_id");
        let sequence = {
            let s: i64 = row.get("sequence");
            s as usize
        };
        let event_type: String = row.get("event_type");
        let event_version: String = row.get("event_version");
        let payload: Value = row.get("payload");
        let metadata: Value = row.get("metadata");
        Ok(SerializedEvent::new(
            aggregate_id,
            sequence,
            aggregate_type,
            event_type,
            event_version,
            payload,
            metadata,
        ))
    }
}

#[async_trait]
impl<'a, A> EventStore<'a, A> for PostgresEventStore
where
    A: 'a + Aggregate + Sync + Send,
{
    async fn load_events(&self, aggregate_id: &str) -> Result<Vec<EventEnvelope<A>>> {
        todo!()
    }

    async fn load_aggregate(&self, aggregate_id: &str) -> anyhow::Result<A> {
        todo!()
    }

    async fn commit(
        &self,
        events: Vec<A::Event<'a>>,
        metadata: HashMap<String, String>,
    ) -> anyhow::Result<Vec<EventEnvelope<A>>> {
        todo!()
    }
}
