use super::{Gendre, Category, Units, Movements};

pub struct CompetitorInfo {
  pub gendre: Gendre,
  pub units: Units,
  pub body_weight: String,
  pub lifted_weight: String,
  pub category: Category,
  pub movements: Movements,
}
