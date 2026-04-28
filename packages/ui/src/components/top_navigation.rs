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
                Link { to: Route::Home {}, {t!("nav-home")} }
                Link { to: Route::About {}, {t!("nav-about")} }
                Link { to: Route::Contact {}, {t!("nav-contact")} }
            }
            ToastRegion { toast }
            DeveloperTools {}
        }
    }
}
