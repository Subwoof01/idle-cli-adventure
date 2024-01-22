use serde::{Deserialize, Serialize};

use crate::stat::Stat;

#[derive(Serialize, Deserialize, Debug)]
pub struct Adventurer {
    pub name: String,
    pub level: u16,
    pub experience: u32,
    pub stats: Vec<Stat>,
}
