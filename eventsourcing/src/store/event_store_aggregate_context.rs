use crate::aggregate::aggregate::Aggregate;
use crate::store::aggregate_context::AggregateContext;

pub struct EventStoreAggregateContext<A: Aggregate> {
    pub aggregate_id: String,
    pub aggregate: A,
    pub current_sequence: usize,
}

impl<A: Aggregate> EventStoreAggregateContext<A> {
    pub(crate) fn context_for(aggregate_id: &str, _is_event_source: bool) -> Self {
        Self {
            aggregate_id: aggregate_id.to_string(),
            aggregate: A::default(),
            current_sequence: 0,
        }
    }
}

impl<A: Aggregate> AggregateContext<A> for EventStoreAggregateContext<A> {
    fn aggregate(&self) -> &A {
        &self.aggregate
    }
}
