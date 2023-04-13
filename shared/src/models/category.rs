use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub enum Category {
    Raw,
    Equipped,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Category::Raw => write!(f, "Raw/Classic"),
            Category::Equipped => write!(f, "Equipped"),
        }
    }
}
