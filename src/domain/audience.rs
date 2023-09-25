use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Audience {
    pub id: Uuid
}