use crate::aggregate::aggregate::Aggregate;
use crate::event::serialized_event::SerializedEvent;
use std::collections::HashMap;

#[derive(Debug)]
pub struct EventEnvelope<A>
where
    A: Aggregate,
{
    pub aggregate_id: String,
    pub sequence: usize,
    pub payload: A::Event,
    pub metadata: HashMap<String, String>,
}

impl<A: Aggregate> Clone for EventEnvelope<A> {
    fn clone(&self) -> Self {
        Self {
            aggregate_id: self.aggregate_id.clone(),
            sequence: self.sequence,
            payload: self.payload.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

impl<A: Aggregate> TryFrom<SerializedEvent> for EventEnvelope<A> {
    type Error = anyhow::Error;

    fn try_from(event: SerializedEvent) -> Result<Self, Self::Error> {
        let payload = serde_json::from_value(event.payload)?;
        let metadata = serde_json::from_value(event.metadata)?;
        Ok(Self {
            aggregate_id: event.aggregate_id,
            sequence: event.sequence,
            payload,
            metadata,
        })
    }
}
