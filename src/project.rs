use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Project {
    
}

impl Project {
    pub fn empty() -> Self {
        Self {}
    }
}