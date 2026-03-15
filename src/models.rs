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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_mutation() {
        let mut state = GameState {
            cards: vec![
                Card { id: 0, image_path: "A".into(), is_flipped: false, is_matched: false },
                Card { id: 1, image_path: "A".into(), is_flipped: false, is_matched: false },
            ],
            score: 0,
            attempts: 0,
        };

        // Simulate flip
        state.cards[0].is_flipped = true;
        assert!(state.cards[0].is_flipped);

        // Simulate match
        state.cards[0].is_matched = true;
        state.cards[1].is_matched = true;
        state.score += 10;
        state.attempts += 1;

        assert_eq!(state.score, 10);
        assert_eq!(state.attempts, 1);
        assert!(state.cards[1].is_matched);
    }
}
