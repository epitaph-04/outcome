use crate::aggregate::aggregate::Aggregate;
use crate::projection::view::View;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ViewRepository<V, A>: Send + Sync
where
    V: View<A>,
    A: Aggregate,
{
    async fn load(&self, view_id: &str) -> Result<Option<V>>;

    async fn load_with_context(&self, view_id: &str) -> Result<Option<(V, ViewContext)>>;

    async fn update_view(&self, view: V, context: ViewContext) -> Result<()>;
}

pub struct ViewContext {
    pub view_instance_id: String,
    pub version: i64,
}

impl ViewContext {
    pub fn new(view_instance_id: String, version: i64) -> Self {
        Self {
            view_instance_id,
            version,
        }
    }
}
