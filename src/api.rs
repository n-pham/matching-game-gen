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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle_cards_length() {
        let cards = shuffle_cards_logic();
        assert_eq!(cards.len(), 12, "Should have 12 cards for a 4x3 grid");
    }

    #[test]
    fn test_shuffle_cards_pairs() {
        let cards = shuffle_cards_logic();
        for card in &cards {
            let matches = cards.iter().filter(|c| c.image_path == card.image_path).count();
            assert_eq!(matches, 2, "Each card image should appear exactly twice (as a pair)");
        }
    }

    #[test]
    fn test_shuffle_cards_randomness() {
        let cards1 = shuffle_cards_logic();
        let cards2 = shuffle_cards_logic();
        // It is statistically possible but highly unlikely for two shuffles to be identical
        let paths1: Vec<_> = cards1.iter().map(|c| &c.image_path).collect();
        let paths2: Vec<_> = cards2.iter().map(|c| &c.image_path).collect();
        assert_ne!(paths1, paths2, "Shuffles should be random");
    }

    #[test]
    fn test_assets_exist_on_disk() {
        // This test ensures that the paths we use (whether through asset! or raw strings)
        // actually point to files that exist in the assets directory.
        let cards = shuffle_cards_logic();
        for card in cards {
            // asset! macro in Dioxus 0.7 might return a URL-like string in some modes,
            // but in dev/test it usually contains the path or we can at least check the source.
            // Since we know they are in the 'assets' folder:
            let filename = card.image_path.split('/').last().unwrap();
            let path = std::path::Path::new("assets").join(filename);
            assert!(path.exists(), "Asset file should exist at {:?}", path);
        }
    }

    #[test]
    fn test_dioxus_toml_config() {
        let toml_str = std::fs::read_to_string("Dioxus.toml").expect("Dioxus.toml missing");
        assert!(toml_str.contains("asset_dir = \"assets\""), "Dioxus.toml should specify 'assets' as the asset directory");
    }

    #[test]
    fn test_main_rs_config_sanity() {
        let main_content = std::fs::read_to_string("src/main.rs").expect("src/main.rs missing");
        
        // We found that manually setting with_resource_directory(current_dir) broke asset loading
        // in Dioxus 0.7 Desktop. The default Config handles it better when using asset! macro.
        assert!(
            !main_content.contains(".with_resource_directory("),
            "src/main.rs should NOT use .with_resource_directory() as it was found to cause 404s in Desktop mode"
        );
        
        assert!(
            main_content.contains("Config::new()"),
            "src/main.rs should use Config::new() for Desktop launch"
        );
    }
}
