pub trait CommandHandler<TCommand, TAggregate> {
    type AggregateStore;
    
    fn handle(&self, command: TCommand);
}