use derivative::Derivative;
use serde::{Deserialize, Serialize};

trait Boostable {
    fn final_value(&self) -> u32;
}

#[derive(Derivative, Serialize, Deserialize, Debug)]
#[derivative(Default)]
pub struct Stat {
    pub name: String,

    #[derivative(Default(value = "0"))]
    base_value: u32,
}

impl Boostable for Stat {
    fn final_value(&self) -> u32 {
        0
    }
}

impl Stat {
    pub fn new(name: &str, base_value: u32) -> Self {
        Self {
            name: name.to_string(),
            base_value,
        }
    }

    pub fn get(&self) -> u32 {
        self.base_value
    }

    pub fn set(&mut self, value: u32) {
        self.base_value = value;
    }
}
