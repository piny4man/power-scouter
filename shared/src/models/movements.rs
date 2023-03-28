use std::fmt;

#[derive(Clone)]
pub enum Movements {
    FullMeet,
    BenchOnly,
}

impl fmt::Display for Movements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Movements::FullMeet => write!(f, "Full meet"),
            Movements::BenchOnly => write!(f, "Bench only"),
        }
    }
}
