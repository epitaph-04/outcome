use crate::domain::audience::Audience;
use crate::domain::kpi::Kpi;
use crate::domain::reach_goal::Goal;
use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use eventsourcing::aggregate::Aggregate;
use eventsourcing::event::DomainEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum StatisticsEvent {
    StatisticsCreated(Uuid, String),
    PrimaryAudienceAdded(Audience, String),
    EvaluationAudiencesAdded(Vec<Audience>, String),
    ReachGoalsAdded(HashMap<Audience, Goal>, String),
}

impl DomainEvent for StatisticsEvent {
    fn event_type(&self) -> String {
        match self {
            StatisticsEvent::StatisticsCreated(_, _) => String::from("StatisticsCreated"),
            StatisticsEvent::PrimaryAudienceAdded(_, _) => String::from("PrimaryAudienceAdded"),
            StatisticsEvent::EvaluationAudiencesAdded(_, _) => {
                String::from("EvaluationAudiencesAdded")
            }
            StatisticsEvent::ReachGoalsAdded(_, _) => String::from("ReachGoalsAdded"),
        }
    }

    fn event_version(&self) -> String {
        todo!()
    }
}

pub enum StatisticsCommand {
    AddPrimaryAudience(Audience, String),
    AddEvaluationAudiences(Vec<Audience>, String),
    AddReachGoals(HashMap<Audience, Goal>, String),
}

#[derive(Default)]
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

    fn add_primary_audience(&mut self, audience: Audience, modified_by: &str) -> Result<()> {
        if self.version == 0 {
            bail!(
                "Cannot apply a command to an un-initialized aggregate. Did you forget something?"
            )
        }
        self.primary_audience = Some(audience);
        self.modified_by = modified_by.to_string();
        self.modified_at = Utc::now();
        Ok(())
    }

    fn add_evaluation_audiences(
        &mut self,
        audiences: Vec<Audience>,
        modified_by: &str,
    ) -> Result<()> {
        if self.version == 0 {
            bail!(
                "Cannot apply a command to an un-initialized aggregate. Did you forget something?"
            )
        }
        self.audiences.extend(audiences);
        self.modified_by = modified_by.to_string();
        self.modified_at = Utc::now();
        Ok(())
    }

    fn add_kpis(&mut self, kpis: HashMap<Audience, Vec<Kpi>>) -> Result<()> {
        if self.version == 0 {
            bail!(
                "Cannot apply a command to an un-initialized aggregate. Did you forget something?"
            )
        }
        for kpi in kpis.into_iter() {
            self.kpis.insert(kpi.0, kpi.1);
        }
        Ok(())
    }

    fn add_reach_goals(&mut self, goals: HashMap<Audience, Goal>, modified_by: &str) -> Result<()> {
        if self.version == 0 {
            bail!(
                "Cannot apply a command to an un-initialized aggregate. Did you forget something?"
            )
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
    type Event = StatisticsEvent;
    type Command = StatisticsCommand;

    fn aggregate_type() -> String {
        String::from("Statistics")
    }

    fn version(&self) -> u64 {
        self.version
    }

    fn handle(&mut self, command: Self::Command) -> Result<Vec<Self::Event>>
    where
        Self: Sized,
    {
        todo!()
    }

    fn apply(&mut self, event: Self::Event) -> Result<()> {
        match event {
            StatisticsEvent::StatisticsCreated(id, creator) => {
                self.id = id;
                self.created_by = creator;
                Ok(())
            }
            StatisticsEvent::PrimaryAudienceAdded(audience, modified_by) => {
                self.add_primary_audience(audience, modified_by.as_str())
            }
            StatisticsEvent::EvaluationAudiencesAdded(audiences, modified_by) => {
                self.add_evaluation_audiences(audiences, modified_by.as_str())
            }
            StatisticsEvent::ReachGoalsAdded(audience_per_goal, modified_by) => {
                self.add_reach_goals(audience_per_goal, modified_by.as_str())
            }
        }
    }
}
