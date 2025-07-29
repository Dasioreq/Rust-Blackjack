use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings
{
    pub cards: [[Vec<String>; 13]; 4],
    pub reverse: Vec<String>,
    pub decks: u8,
    pub dealer_action_interval: f32,
    pub dealer_player_offset: i8,
    pub card_offset: isize
}

impl Settings
{
    pub fn load_from_files(settings_filename: & str) -> Self
    {
        let settings_source = fs::read_to_string(settings_filename)
            .expect("Can't open settings.json");
        let settings: Settings = serde_json::from_str(&settings_source)
            .expect("Can't parse settings.json");

        settings
    }
}