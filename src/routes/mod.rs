use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

mod home;
pub use home::Home;

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},
}
