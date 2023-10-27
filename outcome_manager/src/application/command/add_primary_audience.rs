use anyhow::Result;
use async_trait::async_trait;
use eventsourcing::aggregate::aggregate::Aggregate;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::audience::Audience;
use crate::domain::statistics::{StatisticsAggregate, StatisticsCommand};
use eventsourcing::command::command_handler::CommandHandler;
use eventsourcing::store::event_store::EventStore;

pub struct AddPrimaryAudienceCommand {
    id: String,
    audience_id: Uuid,
    modified_by: String,
}

pub struct AddPrimaryAudienceCommandHandler {
    event_store: Arc<dyn EventStore<StatisticsAggregate, AC = ()>>,
}

impl AddPrimaryAudienceCommandHandler {
    pub fn new(event_store: Arc<dyn EventStore<StatisticsAggregate, AC = ()>>) -> Self {
        Self { event_store }
    }
}

#[async_trait]
impl CommandHandler<AddPrimaryAudienceCommand, StatisticsAggregate>
    for AddPrimaryAudienceCommandHandler
{
    async fn handle(&self, command: AddPrimaryAudienceCommand) -> Result<()> {
        let mut aggregate = self
            .event_store
            .load_aggregate(command.id.as_str())
            .await?
            .aggregate();
        let events = aggregate.handle(StatisticsCommand::AddPrimaryAudience(
            Audience {
                id: command.audience_id,
            },
            command.modified_by,
        ))?;
        self.aggregate_store.commit(events).await?;
        Ok(())
    }
}
