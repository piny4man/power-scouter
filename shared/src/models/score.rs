use serde::{Deserialize, Serialize};

use super::{Category, Gendre, Movements, Units};

#[derive(Serialize, Deserialize, Clone)]
pub struct Score {
    pub body_weight: f64,
    pub lifted_weight: f64,
    pub gendre: Gendre,
    pub category: Category,
    pub movements: Movements,
    pub unit: Units,
    pub ipfgl: f64,
    pub ipf: f64,
    pub old_wilks: f64,
    pub new_wilks: f64,
    pub dots: f64,
}
