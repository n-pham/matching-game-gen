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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scoreboard_render() {
        let mut dom = VirtualDom::new_with_props(ScoreBoard, ScoreBoardProps { score: 10, attempts: 5 });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Score: 10"));
        assert!(html.contains("Attempts: 5"));
    }

    #[test]
    fn test_card_component_render() {
        #[allow(non_snake_case)]
        fn TestWrapper() -> Element {
            let card = Card { id: 1, image_path: "test.svg".into(), is_flipped: true, is_matched: false };
            rsx! {
                CardComponent { 
                    card: card, 
                    on_click: |_| {} 
                }
            }
        }
        let mut dom = VirtualDom::new(TestWrapper);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("test.svg"));
    }
}
