use crate::event::event::DomainEvent;
use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Aggregate: Default + Send + Sync + Serialize + DeserializeOwned {
    type Event: DomainEvent;
    type Command;

    fn aggregate_type() -> String;
    fn handle(&mut self, command: Self::Command) -> Result<Vec<Self::Event>>;
    fn apply(&mut self, event: Self::Event) -> Result<()>;
}
