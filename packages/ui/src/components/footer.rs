use dioxus::prelude::*;
use dioxus_i18n::t;

const FOOTER_CSS: Asset = asset!("/assets/styling/footer.css");

#[component]
pub fn Footer() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: FOOTER_CSS }

        footer { class: "app-footer text-center text-[13px] leading-[1.45]",
            {t!("footer-rights")}
        }
    }
}
