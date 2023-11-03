use crate::aggregate::aggregate::Aggregate;
use crate::event::event_envelop::EventEnvelope;
use crate::projection::error::PersistenceError;
use crate::projection::projection::Projection;
use crate::projection::view::View;
use crate::projection::view_repository::{ViewContext, ViewRepository};
use anyhow::Result;
use async_trait::async_trait;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct GenericProjection<R, V, A>
where
    R: ViewRepository<V, A>,
    V: View<A>,
    A: Aggregate,
{
    view_repository: Arc<R>,
    error_handler: Option<Box<QueryErrorHandler>>,
    phantom: PhantomData<(V, A)>,
}

impl<R, V, A> GenericProjection<R, V, A>
where
    R: ViewRepository<V, A>,
    V: View<A>,
    A: Aggregate,
{
    pub fn new(view_repository: Arc<R>) -> Self {
        Self {
            view_repository,
            error_handler: None,
            phantom: PhantomData::default(),
        }
    }

    pub fn use_error_handler(&mut self, error_handler: Box<QueryErrorHandler>) {
        self.error_handler = Some(error_handler);
    }

    pub async fn load(&self, view_id: &str) -> Option<V> {
        match self.view_repository.load_with_context(view_id).await {
            Ok(option) => option.map(|(view, _)| view),
            Err(e) => {
                if let Some(persistence_error) = e.downcast_ref::<PersistenceError>() {
                    self.handle_error(persistence_error);
                    None
                } else {
                    None
                }
            }
        }
    }

    async fn load_mut(&self, view_id: String) -> Result<(V, ViewContext)> {
        Ok(self
            .view_repository
            .load_with_context(&view_id)
            .await?
            .unwrap_or_else(|| (Default::default(), ViewContext::new(view_id, 0))))
    }

    pub(crate) async fn apply_events(
        &self,
        view_id: &str,
        events: &[EventEnvelope<A>],
    ) -> Result<()> {
        let (mut view, view_context) = self.load_mut(view_id.to_string()).await?;
        for event in events {
            view.update(event);
        }
        self.view_repository.update_view(view, view_context).await?;
        Ok(())
    }

    fn handle_error(&self, error: &PersistenceError) {
        if let Some(handler) = &self.error_handler {
            (handler)(error);
        }
    }
}

#[async_trait]
impl<R, V, A> Projection<A> for GenericProjection<R, V, A>
where
    R: ViewRepository<V, A>,
    V: View<A>,
    A: Aggregate,
{
    async fn dispatch(&self, view_id: &str, events: &[EventEnvelope<A>]) {
        if let Err(err) = self.apply_events(view_id, events).await {
            if let Some(persistence_error) = err.downcast_ref::<PersistenceError>() {
                self.handle_error(persistence_error);
            }
        };
    }
}

pub type QueryErrorHandler = dyn Fn(&PersistenceError) + Send + Sync + 'static;
