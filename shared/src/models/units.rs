use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub enum Units {
    Kg,
    Lb,
}

impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Units::Kg => write!(f, "Kilograms (kg)"),
            Units::Lb => write!(f, "Pounds (lb)"),
        }
    }
}
