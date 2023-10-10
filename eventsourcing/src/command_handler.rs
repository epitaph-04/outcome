pub trait CommandHandler<TCommand, TAggregate> {
    fn handle(&self, command: TCommand);
}
