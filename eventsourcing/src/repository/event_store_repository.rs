use crate::aggregate::aggregate::Aggregate;
use crate::event::serialized_event::SerializedEvent;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait EventStoreRepository: Send + Sync {
    async fn get_events<A: Aggregate>(&self, aggregate_id: &str) -> Result<Vec<SerializedEvent>>;
    async fn get_last_events<A: Aggregate>(
        &self,
        aggregate_id: &str,
        last_sequence: usize,
    ) -> Result<Vec<SerializedEvent>>;
    async fn persist<A: Aggregate>(&self, events: &[SerializedEvent]) -> Result<()>;
}
