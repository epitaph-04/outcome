use crate::aggregate::aggregate::Aggregate;
use crate::event::serialized_event::SerializedEvent;
use crate::repository::event_store_repository::EventStoreRepository;
use crate::repository::sql_query_factory::SqlQueryFactory;
use anyhow::Result;
use async_trait::async_trait;
use futures::TryStreamExt;
use serde_json::Value;
use sqlx::postgres::PgRow;
use sqlx::{Pool, Postgres, Row, Transaction};

const DEFAULT_EVENT_TABLE: &str = "events";

pub struct PostgresEventStoreRepository {
    pool: Pool<Postgres>,
    query_factory: SqlQueryFactory,
}

impl PostgresEventStoreRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            pool,
            query_factory: SqlQueryFactory::new(DEFAULT_EVENT_TABLE),
        }
    }
}

impl PostgresEventStoreRepository {
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
            result.push(PostgresEventStoreRepository::deserialize_event(row)?);
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

    async fn insert_events<A: Aggregate>(&self, events: &[SerializedEvent]) -> Result<()> {
        let mut tx: Transaction<'_, Postgres> = sqlx::Acquire::begin(&self.pool).await?;
        self.persist_events::<A>(self.query_factory.insert_event(), &mut tx, events)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn persist_events<A: Aggregate>(
        &self,
        insert_event_query: &str,
        tx: &mut Transaction<'_, Postgres>,
        events: &[SerializedEvent],
    ) -> Result<usize> {
        let mut current_sequence: usize = 0;
        for event in events {
            current_sequence = event.sequence;
            let event_type = &event.event_type;
            let event_version = &event.event_version;
            let payload = serde_json::to_value(&event.payload)?;
            let metadata = serde_json::to_value(&event.metadata)?;
            sqlx::query(insert_event_query)
                .bind(A::aggregate_type())
                .bind(event.aggregate_id.as_str())
                .bind(event.sequence as i32)
                .bind(event_type)
                .bind(event_version)
                .bind(&payload)
                .bind(&metadata)
                .execute(&mut **tx)
                .await?;
        }
        Ok(current_sequence)
    }
}

#[async_trait]
impl EventStoreRepository for PostgresEventStoreRepository {
    async fn get_events<A: Aggregate>(&self, aggregate_id: &str) -> Result<Vec<SerializedEvent>> {
        self.select_events::<A>(aggregate_id, self.query_factory.select_events())
            .await
    }

    async fn get_last_events<A: Aggregate>(
        &self,
        aggregate_id: &str,
        last_sequence: usize,
    ) -> Result<Vec<SerializedEvent>> {
        let query = self.query_factory.get_last_events(last_sequence);
        self.select_events::<A>(aggregate_id, &query).await
    }

    async fn persist<A: Aggregate>(&self, events: &[SerializedEvent]) -> Result<()> {
        self.insert_events::<A>(events).await
    }
}
