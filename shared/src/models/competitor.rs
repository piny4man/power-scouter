use serde::{Serialize, Deserialize};

use super::{Category, Gendre, Movements, Units};

#[derive(Serialize, Deserialize)]
pub struct CompetitorInfo {
    pub gendre: Gendre,
    pub units: Units,
    pub body_weight: String,
    pub lifted_weight: String,
    pub category: Category,
    pub movements: Movements,
}
