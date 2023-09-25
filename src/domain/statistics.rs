use std::collections::HashMap;
use uuid::Uuid;
use crate::domain::audience::Audience;
use crate::domain::kpi::Kpi;
use chrono::{DateTime, Utc};
use crate::domain::reach_goal::Goal;

pub struct Statistics {
    pub id: Uuid,
    pub created_by: String,
    pub modified_by: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub audiences: Vec<Audience>,
    pub stats: HashMap<Audience, Vec<Kpi>>,
    pub primary_audience: Option<Audience>,
    pub reach_goals: HashMap<Audience, Goal>,
}

impl Statistics {
    pub fn new(id: Uuid, created_by: &str) -> Self {
        Self {
            id,
            created_by: created_by.to_string(),
            modified_by: created_by.to_string(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
            audiences: vec![],
            stats: HashMap::new(),
            primary_audience: None,
            reach_goals: HashMap::new(),
        }
    }

    pub fn add_primary_audience(&mut self, audience: Audience, modified_by: &str) {
        self.primary_audience = Some(audience);
        self.modified_by = modified_by.to_string();
        self.modified_at = Utc::now();
    }

    pub fn add_evaluation_audiences(&mut self, audiences: &mut Vec<Audience>, modified_by: &str) {
        self.audiences.append(audiences);
        self.modified_by = modified_by.to_string();
        self.modified_at = Utc::now();
    }

    pub fn add_statistics(&mut self, kpis: HashMap<Audience, Vec<Kpi>>) {
        for kpi in kpis.into_iter() {
            self.stats.insert(kpi.0, kpi.1);
        }
    }

    pub fn add_reach_goals(&mut self, goals: HashMap<Audience, Goal>) {
        for goal in goals.into_iter() {
            self.reach_goals.insert(goal.0, goal.1);
        }
    }
}