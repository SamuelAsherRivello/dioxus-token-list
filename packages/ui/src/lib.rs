//! This crate contains all shared UI for the workspace.

use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
    #[route("/")]
    Home {},

    #[route("/video")]
    Video {},

    #[route("/about")]
    About {},

    #[route("/contact")]
    Contact {},
}

#[component]
fn AppLayout() -> Element {
    let theme = use_signal(services::storage_service::load_theme);
    let language = use_signal(services::storage_service::load_language);
    let initial_language = language();
    use_init_i18n(|| services::localization_service::config(initial_language));

    let token_load_request = use_signal(components::main::TokenLoadRequest::initial);
    let token_load_cache = use_signal(|| None::<Result<models::TokenLoadResult, String>>);
    let toast = use_signal(|| None::<components::toast::Toast>);

    use_context_provider(|| theme);
    use_context_provider(|| language);
    use_context_provider(|| token_load_request);
    use_context_provider(|| token_load_cache);
    use_context_provider(|| toast);

    let shell_class = format!("app-shell {}", theme().class_name());

    rsx! {
        div { class: "{shell_class}",
            TopNavigation {}
            PageStack {}
            Footer {}
        }
    }
}

#[component]
fn PageStack() -> Element {
    rsx! {
        div {
            class: "page-stack",
            style: "position: relative; isolation: isolate;",
            Page { route: Route::Home {}, will_preload: true,
                Home {}
            }
            Page { route: Route::Video {}, will_preload: true,
                Video {}
            }
            Page { route: Route::About {}, will_preload: true,
                About {}
            }
            Page { route: Route::Contact {}, will_preload: true,
                Contact {}
            }
        }
    }
}

mod app;
pub use app::App;

mod pages {
    pub mod about;
    pub mod contact;
    pub mod home;
    pub mod video;
}
pub use pages::about::About;
pub use pages::contact::Contact;
pub use pages::home::Home;
pub use pages::video::Video;

mod components {
    pub mod developer_tools;
    pub mod footer;
    pub mod main;
    pub mod page;
    pub mod toast;
    pub mod token_list;
    pub mod token_list_item;
    pub mod top_navigation;
}
pub use components::developer_tools::DeveloperTools;
pub use components::footer::Footer;
pub use components::main::{Main, TokenLoadRequest};
pub use components::page::Page;
pub use components::token_list::TokenList;
pub use components::token_list_item::TokenListItem;
pub use components::top_navigation::TopNavigation;
mod models;
pub use models::Token;
mod services;
pub use services::localization_service::AppLanguage;
pub use services::storage_service::Theme;
