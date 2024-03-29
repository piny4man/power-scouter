use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub enum Gendre {
    Male,
    Female,
}

impl fmt::Display for Gendre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gendre::Male => write!(f, "Male"),
            Gendre::Female => write!(f, "Female"),
        }
    }
}
