use crate::aggregate::Aggregate;
use crate::event_envelop::EventEnvelope;
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait EventStore<'a, A>
where
    A: 'a + Aggregate + Send + Sync,
{
    async fn load_events(&self, aggregate_id: &str) -> Result<Vec<EventEnvelope<A>>>;

    async fn load_aggregate(&self, aggregate_id: &str) -> Result<A>;

    async fn commit(
        &self,
        events: Vec<A::Event<'a>>,
        metadata: HashMap<String, String>,
    ) -> Result<Vec<EventEnvelope<A>>>;
}
