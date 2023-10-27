use crate::aggregate::aggregate::Aggregate;
use crate::event::event::DomainEvent;
use crate::event::event_envelop::EventEnvelope;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SerializedEvent {
    pub aggregate_id: String,
    pub sequence: usize,
    pub aggregate_type: String,
    pub event_type: String,
    pub event_version: String,
    pub payload: Value,
    pub metadata: Value,
}

impl SerializedEvent {
    pub fn new(
        aggregate_id: String,
        sequence: usize,
        aggregate_type: String,
        event_type: String,
        event_version: String,
        payload: Value,
        metadata: Value,
    ) -> Self {
        Self {
            aggregate_id,
            sequence,
            aggregate_type,
            event_type,
            event_version,
            payload,
            metadata,
        }
    }
}

impl<A: Aggregate> TryFrom<&EventEnvelope<A>> for SerializedEvent {
    type Error = anyhow::Error;

    fn try_from(event: &EventEnvelope<A>) -> Result<Self> {
        let aggregate_type = A::aggregate_type();
        let event_type = event.payload.event_type();
        let event_version = event.payload.event_version();
        let payload = serde_json::to_value(&event.payload)?;
        let metadata = serde_json::to_value(&event.metadata)?;
        Ok(Self {
            aggregate_id: event.aggregate_id.clone(),
            sequence: event.sequence,
            aggregate_type,
            event_type,
            event_version,
            payload,
            metadata,
        })
    }
}
