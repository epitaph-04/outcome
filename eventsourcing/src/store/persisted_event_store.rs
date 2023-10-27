use crate::aggregate::aggregate::Aggregate;
use crate::event::event_envelop::EventEnvelope;
use crate::event::serialized_event::SerializedEvent;
use crate::repository::event_store_repository::EventStoreRepository;
use crate::store::event_store::EventStore;
use crate::store::event_store_aggregate_context::EventStoreAggregateContext;
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct PersistedEventStore<R, A>
where
    R: EventStoreRepository,
    A: Aggregate,
{
    repository: R,
    _phantom: PhantomData<A>,
}

impl<R, A> PersistedEventStore<R, A>
where
    R: EventStoreRepository,
    A: Aggregate,
{
    pub fn new(repository: R) -> Self {
        Self {
            repository,
            _phantom: PhantomData,
        }
    }
}

impl<R, A> PersistedEventStore<R, A>
where
    R: EventStoreRepository,
    A: Aggregate,
{
    fn wrap_events(
        &self,
        aggregate_id: &str,
        last_sequence: usize,
        resultant_events: Vec<A::Event>,
        base_metadata: HashMap<String, String>,
    ) -> Vec<EventEnvelope<A>> {
        let mut sequence = last_sequence;
        let mut wrapped_events: Vec<EventEnvelope<A>> = Vec::new();
        for payload in resultant_events {
            sequence += 1;
            let aggregate_id: String = aggregate_id.to_string();
            let sequence = sequence;
            let metadata = base_metadata.clone();
            wrapped_events.push(EventEnvelope {
                aggregate_id,
                sequence,
                payload,
                metadata,
            });
        }
        wrapped_events
    }
}

#[async_trait]
impl<A, R> EventStore<A> for PersistedEventStore<R, A>
where
    R: EventStoreRepository,
    A: Aggregate + Send + Sync,
{
    type AC = EventStoreAggregateContext<A>;

    async fn load_events(&self, aggregate_id: &str) -> Result<Vec<EventEnvelope<A>>> {
        let serialized_events = self.repository.get_events::<A>(aggregate_id).await?;
        let events = serialized_events
            .into_iter()
            .map(|evt| EventEnvelope::<A>::try_from(evt).unwrap())
            .collect::<Vec<EventEnvelope<A>>>();
        Ok(events)
    }

    async fn load_aggregate(&self, aggregate_id: &str) -> Result<Self::AC> {
        let mut context: EventStoreAggregateContext<A> =
            EventStoreAggregateContext::context_for(aggregate_id, true);
        let events_to_apply: Vec<EventEnvelope<A>> = self.load_events(aggregate_id).await?;
        for envelope in events_to_apply {
            let event = envelope.payload;
            context.aggregate.apply(event)?;
        }
        Ok(context)
    }

    async fn commit(
        &self,
        aggregate_context: Self::AC,
        events: Vec<A::Event>,
        metadata: HashMap<String, String>,
    ) -> Result<Vec<EventEnvelope<A>>> {
        let aggregate_id = aggregate_context.aggregate_id;
        let last_sequence = aggregate_context.current_sequence;

        let wrapped_events = self.wrap_events(&aggregate_id, last_sequence, events, metadata);
        let serialized_events = wrapped_events
            .iter()
            .map(|evt| SerializedEvent::try_from(evt).unwrap())
            .collect::<Vec<SerializedEvent>>();
        self.repository.persist::<A>(&serialized_events).await?;
        Ok(wrapped_events)
    }
}
