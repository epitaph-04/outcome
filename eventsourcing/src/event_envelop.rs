use crate::aggregate::Aggregate;
use crate::serialized_event::SerializedEvent;
use std::collections::HashMap;

#[derive(Debug)]
pub struct EventEnvelope<'a, A>
where
    A: Aggregate + Send + Sync,
{
    pub aggregate_id: String,
    pub sequence: usize,
    pub payload: A::Event<'a>,
    pub metadata: HashMap<String, String>,
}

impl<'a, A: Aggregate> Clone for EventEnvelope<'a, A> {
    fn clone(&self) -> Self {
        Self {
            aggregate_id: self.aggregate_id.clone(),
            sequence: self.sequence,
            payload: self.payload.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

impl<'a, A: Aggregate> TryFrom<SerializedEvent> for EventEnvelope<'a, A> {
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
