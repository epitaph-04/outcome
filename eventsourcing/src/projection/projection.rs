use crate::aggregate::aggregate::Aggregate;
use crate::event::event_envelop::EventEnvelope;
use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait Projection<A: Aggregate>: Send + Sync {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<A>]);
}
