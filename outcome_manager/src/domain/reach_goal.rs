use crate::domain::kpi::{Cost, Grp, ImpressionX000, Reach1Plus, Reach2Plus, Reach3Plus};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Goal {
    pub grp: Option<Grp>,
    pub impression_x000: Option<ImpressionX000>,
    pub cost: Option<Cost>,
    pub reach_1_plus: Option<Reach1Plus>,
    pub reach_2_plus: Option<Reach2Plus>,
    pub reach_3_plus: Option<Reach3Plus>,
}
