use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt;
pub trait DomainEvent:
    Serialize + DeserializeOwned + Clone + PartialEq + fmt::Debug + Sync + Send
{
    fn event_type(&self) -> String;
    fn event_version(&self) -> String;
}
