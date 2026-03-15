use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct Card {
    pub id: i32,
    pub image_path: String,
    pub is_flipped: bool,
    pub is_matched: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub cards: Vec<Card>,
    pub score: i32,
    pub attempts: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct HighScore {
    pub id: i32,
    pub player_name: String,
    pub score: i32,
    pub time_seconds: i32,
}
