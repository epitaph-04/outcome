use std::sync::Arc;
use uuid::Uuid;

use crate::application::store::Store;
use crate::application::command_handler::CommandHandler;

pub struct AddPrimaryAudienceCommand {
    id: Uuid,
    audience_id: Uuid,
    modified_by: String,
}


impl<AddPrimaryAudienceCommand, Statistics> CommandHandler<AddPrimaryAudienceCommand, Statistics> for AddPrimaryAudienceCommand {
    type AggregateStore = Box<dyn Store<Statistics>>;

    fn handle(&self, command: AddPrimaryAudienceCommand) {

    }
}