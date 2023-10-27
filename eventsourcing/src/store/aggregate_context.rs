use crate::aggregate::aggregate::Aggregate;

pub trait AggregateContext<A>
where
    A: Aggregate,
{
    fn aggregate(&self) -> &A;
}
