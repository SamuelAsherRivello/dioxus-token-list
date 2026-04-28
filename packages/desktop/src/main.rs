use dioxus::prelude::*;

use ui::App as UiApp;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        UiApp {}
    }
}
