use crate::event::DomainEvent;
use anyhow::Result;

pub trait Aggregate: Send + Sync {
    type Event<'a>: 'a + DomainEvent;
    type Command;

    fn aggregate_type() -> String;
    fn version(&self) -> u64;
    fn handle<'a>(&mut self, command: Self::Command) -> Result<Vec<Self::Event<'a>>>
    where
        Self: Sized;
    fn apply<'a>(&mut self, event: Self::Event<'a>) -> Result<()>
    where
        Self: Sized;
}
