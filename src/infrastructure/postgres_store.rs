use uuid::Uuid;
use crate::{application::store::Store, domain::statistics::StatisticsAggregate};


pub struct PostgresStore {

}

impl Store<StatisticsAggregate> for PostgresStore {
    fn get(&self, id: Uuid) -> StatisticsAggregate {
        todo!()
    }

    fn add(&self, entity: StatisticsAggregate) {
        todo!()
    }

    fn remove(&self, entity: StatisticsAggregate) {
        todo!()
    }
}