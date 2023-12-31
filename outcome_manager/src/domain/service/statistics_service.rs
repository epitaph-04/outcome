use crate::domain::service::plan_service::PlanService;
use anyhow::Result;
use uuid::Uuid;

struct StatisticsService {
    plan_service: Box<dyn PlanService>,
}

impl StatisticsService {
    pub fn new(plan_service: Box<dyn PlanService>) -> Self {
        StatisticsService { plan_service }
    }
    pub async fn compute(&self, id: Uuid) -> Result<()> {
        let _plan_details = self.plan_service.get_plan_details(id).await?;
        Ok(())
    }
}
