use dioxus::prelude::*;

use crate::client::Main;

#[component]
pub fn Home() -> Element {
    rsx! {
        Main {}
    }
}
