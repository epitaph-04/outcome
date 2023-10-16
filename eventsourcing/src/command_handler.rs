use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CommandHandler<TCommand, TAggregate> {
    async fn handle(&self, command: TCommand) -> Result<()>;
}
