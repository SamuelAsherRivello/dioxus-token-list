use dioxus::prelude::*;

use super::Route;

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
