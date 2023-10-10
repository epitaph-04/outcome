use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Audience {
    pub id: Uuid,
}
