use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::token_list_item::TokenListItem;
use crate::models::Token;

#[component]
pub fn TokenList(tokens: Vec<Token>) -> Element {
    rsx! {
        div { class: "token-list overflow-x-auto overflow-y-hidden",
            div { class: "token-list__header uppercase select-none",
                span { {t!("token-header-asset")} }
                span { {t!("token-header-price")} }
                span { {t!("token-header-change")} }
                span { {t!("token-header-market-cap")} }
                span { {t!("token-header-volume")} }
                span { {t!("token-header-seven-day")} }
            }
            ol { class: "token-list__items",
                for token in tokens {
                    TokenListItem { key: "{token.id}", token }
                }
            }
        }
    }
}
