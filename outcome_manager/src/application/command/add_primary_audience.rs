use std::sync::Arc;
use uuid::Uuid;

use crate::domain::statistics::StatisticsAggregate;
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
    fn handle(&self, command: AddPrimaryAudienceCommand) {}
}
