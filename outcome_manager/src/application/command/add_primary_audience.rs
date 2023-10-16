use anyhow::Result;
use eventsourcing::aggregate::Aggregate;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::audience::Audience;
use crate::domain::statistics::{StatisticsAggregate, StatisticsCommand};
use eventsourcing::command_handler::CommandHandler;
use eventsourcing::event_store::EventStore;

pub struct AddPrimaryAudienceCommand {
    id: Uuid,
    audience_id: Uuid,
    modified_by: String,
}

pub struct AddPrimaryAudienceCommandHandler {
    aggregate_store: Arc<dyn EventStore<StatisticsAggregate>>,
}

impl AddPrimaryAudienceCommandHandler {
    pub fn new(aggregate_store: Arc<dyn EventStore<StatisticsAggregate>>) -> Self {
        Self { aggregate_store }
    }
}

impl CommandHandler<AddPrimaryAudienceCommand, StatisticsAggregate>
    for AddPrimaryAudienceCommandHandler
{
    async fn handle(&self, command: AddPrimaryAudienceCommand) -> Result<()> {
        let mut aggregate = self.aggregate_store.load_aggregate(command.id).await?;
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
