use crate::aggregate::Aggregate;
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
