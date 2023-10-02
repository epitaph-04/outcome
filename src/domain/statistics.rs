use anyhow::Result;
use std::collections::HashMap;
use uuid::Uuid;
use crate::domain::audience::Audience;
use crate::domain::kpi::Kpi;
use chrono::{DateTime, Utc};
use crate::domain::aggregate;
use crate::domain::aggregate::Aggregate;
use crate::domain::reach_goal::Goal;

pub enum StatisticsEvent {
    PrimaryAudienceAdded(Audience, String),
    EvaluationAudiencesAdded(Vec<Audience>, String),
    ReachGoalsAdded(HashMap<Audience, Goal>, String)
}

pub struct StatisticsAggregate {
    pub id: Uuid,
    version: u64,
    pub created_by: String,
    pub modified_by: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub audiences: Vec<Audience>,
    pub kpis: HashMap<Audience, Vec<Kpi>>,
    pub primary_audience: Option<Audience>,
    pub reach_goals: HashMap<Audience, Goal>,
}

impl StatisticsAggregate {
    pub fn new(id: Uuid, created_by: &str) -> Self {
        Self {
            id,
            version: 1,
            created_by: created_by.to_string(),
            modified_by: created_by.to_string(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
            audiences: vec![],
            kpis: HashMap::new(),
            primary_audience: None,
            reach_goals: HashMap::new(),
        }
    }

    pub fn add_primary_audience(&mut self, audience: Audience, modified_by: &str) -> Result<()> {
        if self.version == 0 {
            Err("Cannot apply a command to an un-initialized aggregate. Did you forget something?".to_owned())
        }
        self.primary_audience = Some(audience);
        self.modified_by = modified_by.to_string();
        self.modified_at = Utc::now();
        Ok(())
    }

    pub fn add_evaluation_audiences(&mut self, audiences: &mut Vec<Audience>, modified_by: &str) -> Result<()> {
        if self.version == 0 {
            Err("Cannot apply a command to an un-initialized aggregate. Did you forget something?".to_owned())
        }
        self.audiences.append(audiences);
        self.modified_by = modified_by.to_string();
        self.modified_at = Utc::now();
        Ok(())
    }

    pub fn add_kpis(&mut self, kpis: HashMap<Audience, Vec<Kpi>>) -> Result<()> {
        if self.version == 0 {
            Err("Cannot apply a command to an un-initialized aggregate. Did you forget something?".to_owned())
        }
        for kpi in kpis.into_iter() {
            self.kpis.insert(kpi.0, kpi.1);
        }
        Ok(())
    }

    pub fn add_reach_goals(&mut self, goals: HashMap<Audience, Goal>, modified_by: &str) -> Result<()> {
        if self.version == 0 {
            Err("Cannot apply a command to an un-initialized aggregate. Did you forget something?".to_owned())
        }
        for goal in goals.into_iter() {
            self.reach_goals.insert(goal.0, goal.1);
        }
        self.modified_by = modified_by.to_string();
        self.modified_at = Utc::now();
        Ok(())
    }
}

impl Aggregate for StatisticsAggregate {
    type Item = StatisticsEvent;

    fn version(&self) -> u64 {
        self.version
    }
    fn apply(&self, evt: &StatisticsEvent) -> StatisticsAggregate {
        let aggregate = StatisticsAggregate {
            version: self.version + 1,
            id: self.id,
            modified_by: self.modified_by.to_string(),
            created_at: Default::default(),
            modified_at: Default::default(),
            audiences: vec![],
            kpis: Default::default(),
            primary_audience: None,
            created_by: "".to_string(),
            reach_goals: Default::default(),
        };

        aggregate
    }
}