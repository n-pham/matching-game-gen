use dioxus::prelude::*;
use crate::models::GameState;
use crate::api::shuffle_cards_logic;
use crate::components::{CardComponent, ScoreBoard};

#[component]
pub fn Home() -> Element {
    let mut game_state = use_signal(|| GameState {
        cards: shuffle_cards_logic(),
        score: 0,
        attempts: 0,
    });

    let flipped_indices = use_signal(|| Vec::<usize>::new());

    let on_card_click = use_callback(move |id: i32| {
        let mut state = game_state.write();
        let idx = state.cards.iter().position(|c| c.id == id).unwrap();

        // Check if we can flip this card
        if state.cards[idx].is_flipped || state.cards[idx].is_matched || flipped_indices.read().len() >= 2 {
            return;
        }

        // Flip it
        state.cards[idx].is_flipped = true;
        let mut flipped_indices = flipped_indices;
        flipped_indices.write().push(idx);

        // If two flipped, check for match
        if flipped_indices.read().len() == 2 {
            state.attempts += 1;
            let indices = flipped_indices.read();
            let idx1 = indices[0];
            let idx2 = indices[1];
            let path1 = state.cards[idx1].image_path.clone();
            let path2 = state.cards[idx2].image_path.clone();
            drop(indices);

            if path1 == path2 {
                state.cards[idx1].is_matched = true;
                state.cards[idx2].is_matched = true;
                state.score += 10;
                flipped_indices.write().clear();
            } else {
                // If no match, hide them after a delay
                drop(state); // Drop game_state lock before spawn
                spawn(async move {
                    futures_timer::Delay::new(std::time::Duration::from_secs(1)).await;
                    let mut game_state = game_state;
                    let mut flipped_indices = flipped_indices;
                    let mut state = game_state.write();
                    state.cards[idx1].is_flipped = false;
                    state.cards[idx2].is_flipped = false;
                    flipped_indices.write().clear();
                });
            }
        }
    });

    rsx! {
        div {
            class: "game-container",
            ScoreBoard { 
                score: game_state.read().score, 
                attempts: game_state.read().attempts 
            }
            div {
                class: "grid",
                for card in game_state.read().cards.iter() {
                    CardComponent { 
                        card: card.clone(), 
                        on_click: move |id| on_card_click.call(id) 
                    }
                }
            }
            button {
                onclick: move |_| {
                    let mut game_state = game_state;
                    game_state.set(GameState {
                        cards: shuffle_cards_logic(),
                        score: 0,
                        attempts: 0,
                    });
                },
                "Restart Game"
            }
        }
        style {
            r#"
            .grid {{
                display: grid;
                grid-template-columns: repeat(4, 1fr);
                grid-template-rows: repeat(3, 1fr);
                gap: 10px;
                max-width: 600px;
                margin: 20px auto;
            }}
            .card {{
                aspect-ratio: 1;
                border: 2px solid #333;
                border-radius: 8px;
                overflow: hidden;
                cursor: pointer;
                background-color: #fff;
                transition: transform 0.2s;
                display: flex;
                align-items: center;
                justify-content: center;
                padding: 5px;
            }}
            .card:hover {{
                transform: scale(1.05);
            }}
            .card.matched {{
                opacity: 0.3;
                cursor: default;
                border: 2px solid green;
            }}
            .card.flipped {{
                border: 2px solid #007bff;
            }}
            .game-container {{
                text-align: center;
                font-family: sans-serif;
                padding: 20px;
                background-color: #f0f0f0;
                min-height: 100vh;
            }}
            .score-board {{
                font-size: 1.5rem;
                margin-bottom: 20px;
                font-weight: bold;
            }}
            button {{
                padding: 10px 20px;
                font-size: 1rem;
                cursor: pointer;
                background-color: #007bff;
                color: white;
                border: none;
                border-radius: 5px;
                margin-top: 20px;
            }}
            "#
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_home_render() {
        let mut dom = VirtualDom::new(Home);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Score: 0"));
        assert!(html.contains("Attempts: 0"));
        assert!(html.contains("Restart Game"));
    }
}
