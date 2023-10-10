use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[async_trait]
pub trait PlanService {
    async fn get_plan_details(&self, id: Uuid) -> Result<PlanDetails>;
}

pub struct PlanDetails {
    id: Uuid,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    buys: Vec<Buy>,
}

pub enum Buy {
    LinearBuy(LinearBuy),
    AddressableBuy(AddressableBuy),
}

pub struct LinearBuy {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub buyable_medium_id: Uuid,
    pub buying_audience_id: Uuid,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub grp: f32,
    pub cost_per_grp: f32,
}

pub struct AddressableBuy {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub buyable_medium_id: Uuid,
    pub buying_audience_id: Uuid,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub impressions_000: f32,
    pub cost_per_thousand_impressions: f32,
    pub frequency_cap: Option<i32>,
}
