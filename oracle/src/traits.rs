use anchor_lang::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
pub struct NFTTraits {
    pub background: u8,      // 0-255: represents different background colors/scenes
    pub mood: u8,           // 0-255: happy, sad, excited, etc.
    pub activity: u8,       // 0-255: sleeping, playing, working, etc.
    pub weather_effect: u8,  // 0-255: sunny, rainy, snowy, etc.
    pub time_of_day: u8,    // 0-255: morning, afternoon, evening, night
    pub special_event: u8,  // 0-255: holiday, celebration, etc.
    pub power_level: u16,   // 0-65535: strength based on external factors
    pub rarity_score: u16,  // 0-65535: calculated rarity score
}

impl NFTTraits {
    pub fn new() -> Self {
        Self {
            background: 0,
            mood: 128,
            activity: 128,
            weather_effect: 0,
            time_of_day: 128,
            special_event: 0,
            power_level: 1000,
            rarity_score: 1000,
        }
    }

    pub fn calculate_rarity_score(&self) -> u16 {
        // Simple rarity calculation based on trait combinations
        let mut score = 0u16;
        score += self.background as u16 * 1000;
        score += self.mood as u16 * 100;
        score += self.activity as u16 * 50;
        score += self.weather_effect as u16 * 75;
        score += self.time_of_day as u16 * 25;
        score += self.special_event as u16 * 200;
        score
    }

    pub fn update_rarity_score(&mut self) {
        self.rarity_score = self.calculate_rarity_score();
    }
}

impl Default for NFTTraits {
    fn default() -> Self {
        Self::new()
    }
}
