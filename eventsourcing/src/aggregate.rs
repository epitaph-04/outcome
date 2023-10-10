use crate::event::DomainEvent;
use anyhow::Result;

pub trait Aggregate {
    type Event: DomainEvent;
    type Command;

    fn aggregate_type() -> String;
    fn version(&self) -> u64;
    fn handle(&mut self, command: Self::Command) -> Result<Vec<Self::Event>>
    where
        Self: Sized;
    fn apply(&mut self, event: Self::Event) -> Result<()>
    where
        Self: Sized;
}
