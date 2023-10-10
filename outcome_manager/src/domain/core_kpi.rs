use crate::domain::audience::Audience;

pub struct CoreKpi {
    pub audience: Audience,
    pub audience_weight: CoreAudienceWeight,
    pub population: CorePopulation,
    pub impression: CoreImpressions,
    pub cost: CoreCost,
    pub contact_distribution_in_impressions: Vec<f32>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct CoreAudienceWeight(pub f32);
#[derive(Debug, Clone, PartialEq)]
pub struct CorePopulation(pub f32);
#[derive(Debug, Clone, PartialEq)]
pub struct CoreImpressions(pub f32);
#[derive(Debug, Clone, PartialEq)]
pub struct CoreCost(pub f32);
