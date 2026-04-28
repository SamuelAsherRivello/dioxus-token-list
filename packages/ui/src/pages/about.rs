use dioxus::prelude::*;
use dioxus_i18n::t;

const ABOUT_CSS: Asset = asset!("/assets/styling/about.css");
const PROFILE_IMAGE: Asset = asset!("/assets/images/profile/samuel-asher-rivello.png");
const RUST_LOGO: Asset = asset!("/assets/images/tech/rust-logo.svg");
const DIOXUS_LOGO: Asset = asset!("/assets/images/tech/dioxus-logo-full.png");

#[component]
pub fn About() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: ABOUT_CSS }

        section { class: "page-content about-page",
            div { class: "about-page__copy",
                h1 { {t!("about-title")} }
                p {
                    {t!("about-intro-1")}
                }
                p {
                    {t!("about-intro-2")}
                }
                p {
                    {t!("about-intro-3")}
                }
            }

            img {
                class: "about-page__portrait",
                src: PROFILE_IMAGE,
                alt: t!("profile-alt"),
            }
        }

        section { class: "page-content about-feature about-feature--rust",
            img {
                class: "about-feature__image about-feature__image--rust",
                src: RUST_LOGO,
                alt: t!("rust-logo-alt"),
            }

            div { class: "about-feature__copy",
                h2 { {t!("rust-title")} }
                p {
                    {t!("rust-copy-1")}
                }
                p {
                    {t!("rust-copy-2")}
                }
            }
        }

        section { class: "page-content about-dioxus",
            div { class: "about-dioxus__image-frame",
                img {
                    class: "about-dioxus__image",
                    src: DIOXUS_LOGO,
                    alt: t!("dioxus-logo-alt"),
                }
            }

            div { class: "about-dioxus__copy",
                h2 { {t!("dioxus-title")} }
                p {
                    {t!("dioxus-copy-1")}
                }
                p {
                    {t!("dioxus-copy-2")}
                }
            }
        }
    }
}
