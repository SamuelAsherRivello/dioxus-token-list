use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::developer_tools::DeveloperTools;
use crate::components::toast::{Toast, ToastRegion};
use crate::Route;

const TOP_NAVIGATION_CSS: Asset = asset!("/assets/styling/top_navigation.css");

#[component]
pub fn TopNavigation() -> Element {
    let toast = use_context::<Signal<Option<Toast>>>();

    rsx! {
        document::Link { rel: "stylesheet", href: TOP_NAVIGATION_CSS }

        header { id: "top-navigation",
            nav {
                div { class: "top-navigation__pages",
                    Link { to: Route::Home {},
                        span { class: "top-navigation__label-full", {t!("nav-home")} }
                        span { class: "top-navigation__label-short", {t!("nav-home-short")} }
                    }
                    Link { to: Route::Video {},
                        span { class: "top-navigation__label-full", {t!("nav-video")} }
                        span { class: "top-navigation__label-short", {t!("nav-video-short")} }
                    }
                    Link { to: Route::About {},
                        span { class: "top-navigation__label-full", {t!("nav-about")} }
                        span { class: "top-navigation__label-short", {t!("nav-about-short")} }
                    }
                    Link { to: Route::Contact {},
                        span { class: "top-navigation__label-full", {t!("nav-contact")} }
                        span { class: "top-navigation__label-short", {t!("nav-contact-short")} }
                    }
                }
                DeveloperTools {}
            }
            ToastRegion { toast }
        }
    }
}
