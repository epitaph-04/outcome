use uuid::Uuid;

pub trait Repository<T> {
    fn get(id: Uuid) -> T;
    fn add(entity: T);
    fn remove(entity: T);
}