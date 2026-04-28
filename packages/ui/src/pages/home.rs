use dioxus::prelude::*;

use crate::Main;

#[component]
pub fn Home() -> Element {
    rsx! {
        Main {}
    }
}
