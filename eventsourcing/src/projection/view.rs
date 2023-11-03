use crate::aggregate::aggregate::Aggregate;
use crate::event::event_envelop::EventEnvelope;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub trait View<A: Aggregate>: Debug + Default + Serialize + DeserializeOwned + Send + Sync {
    fn update(&mut self, event: &EventEnvelope<A>);
}
