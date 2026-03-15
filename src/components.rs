use dioxus::prelude::*;
use crate::models::Card;

#[component]
pub fn CardComponent(card: Card, on_click: EventHandler<i32>) -> Element {
    let src = if card.is_flipped || card.is_matched {
        card.image_path.clone()
    } else {
        asset!("/assets/face-down.svg").to_string()
    };

    let matched_class = if card.is_matched { "matched" } else { "" };
    let flipped_class = if card.is_flipped { "flipped" } else { "" };

    rsx! {
        div {
            class: "card {matched_class} {flipped_class}",
            onclick: move |_| on_click.call(card.id),
            img {
                src: "{src}",
                alt: "Card image",
                width: "100%",
                height: "100%",
            }
        }
    }
}

#[component]
pub fn ScoreBoard(score: i32, attempts: i32) -> Element {
    rsx! {
        div {
            class: "score-board",
            p { "Score: {score}" }
            p { "Attempts: {attempts}" }
        }
    }
}
