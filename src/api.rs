use dioxus::prelude::*;
use crate::models::{HighScore, Card};
use rand::seq::SliceRandom;

// Hard-coded list of assets that exist in the folder (verified previously)
fn get_all_artboards() -> Vec<String> {
    vec![
        asset!("/assets/Artboard 1.svg").to_string(),
        asset!("/assets/Artboard 10.svg").to_string(),
        asset!("/assets/Artboard 11.svg").to_string(),
        asset!("/assets/Artboard 12.svg").to_string(),
        asset!("/assets/Artboard 13.svg").to_string(),
        asset!("/assets/Artboard 14.svg").to_string(),
        asset!("/assets/Artboard 15.svg").to_string(),
        asset!("/assets/Artboard 16.svg").to_string(),
        asset!("/assets/Artboard 17.svg").to_string(),
        asset!("/assets/Artboard 18.svg").to_string(),
        asset!("/assets/Artboard 19.svg").to_string(),
        asset!("/assets/Artboard 20.svg").to_string(),
        asset!("/assets/Artboard 21.svg").to_string(),
        asset!("/assets/Artboard 22.svg").to_string(),
        asset!("/assets/Artboard 23.svg").to_string(),
        asset!("/assets/Artboard 24.svg").to_string(),
        asset!("/assets/Artboard 25.svg").to_string(),
        asset!("/assets/Artboard 26.svg").to_string(),
        asset!("/assets/Artboard 27.svg").to_string(),
        asset!("/assets/Artboard 28.svg").to_string(),
        asset!("/assets/Artboard 29.svg").to_string(),
        asset!("/assets/Artboard 30.svg").to_string(),
        asset!("/assets/Artboard 31.svg").to_string(),
        asset!("/assets/Artboard 32.svg").to_string(),
        asset!("/assets/Artboard 33.svg").to_string(),
        asset!("/assets/Artboard 34.svg").to_string(),
        asset!("/assets/Artboard 35.svg").to_string(),
        asset!("/assets/Artboard 36.svg").to_string(),
        asset!("/assets/Artboard 37.svg").to_string(),
        asset!("/assets/Artboard 38.svg").to_string(),
        asset!("/assets/Artboard 39.svg").to_string(),
        asset!("/assets/Artboard 40.svg").to_string(),
        asset!("/assets/Artboard 41.svg").to_string(),
        asset!("/assets/Artboard 42.svg").to_string(),
        asset!("/assets/Artboard 43.svg").to_string(),
        asset!("/assets/Artboard 44.svg").to_string(),
    ]
}

pub fn shuffle_cards_logic() -> Vec<Card> {
    let mut artboards = get_all_artboards();
    let mut rng = rand::thread_rng();
    artboards.shuffle(&mut rng);
    
    // Take 6 pairs (12 cards total for 4x3 grid)
    let selected = &artboards[0..6];
    let mut cards = Vec::new();
    let mut id = 0;
    for path in selected {
        for _ in 0..2 {
            cards.push(Card {
                id,
                image_path: path.clone(),
                is_flipped: false,
                is_matched: false,
            });
            id += 1;
        }
    }
    cards.shuffle(&mut rng);
    cards
}

#[server]
pub async fn get_high_scores() -> Result<Vec<HighScore>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::server::get_data;
        let data = get_data().lock().map_err(|e| ServerFnError::new(format!("Mutex lock error: {}", e)))?;
        let mut scores = data.clone();
        scores.sort_by(|a, b| b.score.cmp(&a.score));
        Ok(scores.into_iter().take(10).collect())
    }
    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}

#[server]
pub async fn submit_score(player_name: String, score: i32, time_seconds: i32) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::server::get_data;
        let mut data = get_data().lock().map_err(|e| ServerFnError::new(format!("Mutex lock error: {}", e)))?;
        let next_id = data.iter().map(|s| s.id).max().unwrap_or(0) + 1;
        data.push(HighScore {
            id: next_id,
            player_name,
            score,
            time_seconds,
        });
        Ok(())
    }
    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}

#[server]
pub async fn get_shuffled_cards() -> Result<Vec<Card>, ServerFnError> {
    Ok(shuffle_cards_logic())
}
