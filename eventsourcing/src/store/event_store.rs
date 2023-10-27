use crate::aggregate::aggregate::Aggregate;
use crate::event::event_envelop::EventEnvelope;
use crate::store::aggregate_context::AggregateContext;
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait EventStore<A>: Send + Sync
where
    A: Aggregate,
{
    type AC: AggregateContext<A>;

    async fn load_events(&self, aggregate_id: &str) -> Result<Vec<EventEnvelope<A>>>;

    async fn load_aggregate(&self, aggregate_id: &str) -> Result<Self::AC>;

    async fn commit(
        &self,
        context: Self::AC,
        events: Vec<A::Event>,
        metadata: HashMap<String, String>,
    ) -> Result<Vec<EventEnvelope<A>>>;
}
