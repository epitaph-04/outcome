use uuid::Uuid;

pub trait Store<T> {
    fn get(&self, id: Uuid) -> T;
    fn add(&self, entry: T);
    fn remove(&self, entry: T);
}