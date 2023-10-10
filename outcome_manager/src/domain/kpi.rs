use crate::domain::core_kpi::{CoreKpi, CorePopulation};
use crate::domain::currency::Currency;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Money(Currency, f32);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KpiDoubleValue(f32);
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KpiPercentageValue(f32);
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KpiMoneyValue(Money);
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KpiErrorValue(String);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KpiValueVariant<T> {
    Value(T),
    KpiError(KpiErrorValue),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudienceWeight {
    pub value: KpiValueVariant<KpiDoubleValue>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Population {
    pub value: KpiValueVariant<KpiDoubleValue>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Grp {
    pub value: KpiValueVariant<KpiDoubleValue>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImpressionX000 {
    pub value: KpiValueVariant<KpiDoubleValue>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reach1Plus {
    pub value: KpiValueVariant<KpiPercentageValue>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reach2Plus {
    pub value: KpiValueVariant<KpiPercentageValue>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reach3Plus {
    pub value: KpiValueVariant<KpiPercentageValue>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cost {
    pub value: KpiValueVariant<KpiMoneyValue>,
}

impl TryFrom<CoreKpi> for AudienceWeight {
    type Error = &'static str;
    fn try_from(value: CoreKpi) -> Result<Self, Self::Error> {
        if value.audience_weight.0.is_infinite() || value.audience_weight.0.is_nan() {
            return Err("Kpi data is NaN");
        }

        Ok(Self {
            value: KpiValueVariant::Value(KpiDoubleValue(value.audience_weight.0)),
        })
    }
}

impl TryFrom<CoreKpi> for Population {
    type Error = &'static str;
    fn try_from(value: CoreKpi) -> Result<Self, Self::Error> {
        if value.population.0.is_infinite() || value.population.0.is_nan() {
            return Err("Kpi data is NaN");
        }

        Ok(Self {
            value: KpiValueVariant::Value(KpiDoubleValue(value.population.0)),
        })
    }
}

impl TryFrom<CoreKpi> for Grp {
    type Error = &'static str;
    fn try_from(value: CoreKpi) -> Result<Self, Self::Error> {
        let grp = if value.population != CorePopulation(0.0) {
            value.impression.0 / value.population.0 * 100.0
        } else {
            0.0
        };

        Ok(Self {
            value: KpiValueVariant::Value(KpiDoubleValue(grp)),
        })
    }
}

impl TryFrom<CoreKpi> for ImpressionX000 {
    type Error = &'static str;
    fn try_from(value: CoreKpi) -> Result<Self, Self::Error> {
        if value.impression.0.is_infinite() || value.impression.0.is_nan() {
            return Err("Kpi data is NaN");
        }
        Ok(Self {
            value: KpiValueVariant::Value(KpiDoubleValue(value.impression.0 / 1000f32)),
        })
    }
}

impl TryFrom<CoreKpi> for Reach1Plus {
    type Error = &'static str;
    fn try_from(value: CoreKpi) -> Result<Self, Self::Error> {
        Ok(Self {
            value: KpiValueVariant::Value(KpiPercentageValue(calculate_reach(
                &value.contact_distribution_in_impressions,
                1,
            ))),
        })
    }
}

impl TryFrom<CoreKpi> for Reach2Plus {
    type Error = &'static str;
    fn try_from(value: CoreKpi) -> Result<Self, Self::Error> {
        Ok(Self {
            value: KpiValueVariant::Value(KpiPercentageValue(calculate_reach(
                &value.contact_distribution_in_impressions,
                2,
            ))),
        })
    }
}

impl TryFrom<CoreKpi> for Reach3Plus {
    type Error = &'static str;
    fn try_from(value: CoreKpi) -> Result<Self, Self::Error> {
        Ok(Self {
            value: KpiValueVariant::Value(KpiPercentageValue(calculate_reach(
                &value.contact_distribution_in_impressions,
                3,
            ))),
        })
    }
}

impl TryFrom<CoreKpi> for Cost {
    type Error = &'static str;
    fn try_from(value: CoreKpi) -> Result<Self, Self::Error> {
        if value.cost.0.is_infinite() || value.cost.0.is_nan() {
            return Err("Kpi data is NaN");
        }
        Ok(Self {
            value: KpiValueVariant::Value(KpiMoneyValue(Money(Currency::USD, value.cost.0))),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Kpi {
    AudienceWeight(AudienceWeight),
    Population(Population),
    Grp(Grp),
    ImpressionX000(ImpressionX000),
    Reach1Plus(Reach1Plus),
    Reach2Plus(Reach2Plus),
    Reach3Plus(Reach3Plus),
    Cost(Cost),
}

fn calculate_reach(contract_distribution: &Vec<f32>, reach: usize) -> f32 {
    let length = 13;
    let mut n_plus_contacts_distribution = vec![0f32; length];
    n_plus_contacts_distribution[0] = 1.0;

    if contract_distribution.len() > 0 {
        for i in 1..length.min(contract_distribution.len()) {
            n_plus_contacts_distribution[i] =
                n_plus_contacts_distribution[i - 1] - contract_distribution[i - 1];
        }
    }
    n_plus_contacts_distribution[reach]
}
