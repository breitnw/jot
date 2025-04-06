use core::fmt;
use serde::{Deserialize, Serialize};

/// A single note and its metadata
#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub note_id: u32,
    pub user_id: u32,
    pub text: String,
    pub timestamp: String,
    pub priority: Priority,
    pub dismissed: bool,
}

/// The priority assigned to a note: either low, medium, or high
#[derive(Serialize, Deserialize, Debug)]
pub enum Priority {
    LOW,
    MED,
    HIGH,
}

impl TryFrom<u32> for Priority {
    type Error = ();
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == Priority::LOW as u32 => Ok(Priority::LOW),
            x if x == Priority::MED as u32 => Ok(Priority::MED),
            x if x == Priority::HIGH as u32 => Ok(Priority::HIGH),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                Self::HIGH => "HIGH",
                Self::MED => "MED",
                Self::LOW => "LOW",
            }
        )
    }
}
