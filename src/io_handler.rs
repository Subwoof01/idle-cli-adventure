use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

use crate::adventurer::Adventurer;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub adventurers: Vec<Adventurer>,
}

pub fn load_data(data: &mut Data) -> color_eyre::Result<()> {
    // TODO: Adventurer should be single as the game runs only once
    // when it is called in the terminal.
    // Game data--like monsters, items, etc--should be loaded in a loop.
    // Perhaps have some kind of parsing for which data is important,
    // and structure the directories like that to reduce IO calls
    // and only open the streams for the needed files?
    let mut file = File::open("data/steven.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let json: Adventurer = serde_json::from_str(&contents)?;

    data.adventurers.push(json);

    Ok(())
}
