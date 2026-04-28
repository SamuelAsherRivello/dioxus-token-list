use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::main::{
    format_currency, format_optional_currency, format_percent, local_token_icon,
};
use crate::models::Token;

#[component]
pub fn TokenListItem(token: Token) -> Element {
    let rank = token.market_cap_rank.unwrap_or_default();
    let price_change = token.price_change_percentage_24h.unwrap_or_default();
    let change_class = if price_change >= 0.0 {
        "token-list-item__change token-list-item__change--up"
    } else {
        "token-list-item__change token-list-item__change--down"
    };
    let coingecko_url = format!("https://www.coingecko.com/en/coins/{}", token.id);
    let logo_alt = t!("token-logo-alt", name: token.name.clone());

    rsx! {
        li { class: "token-list-item",
            div { class: "token-list-item__asset",
                span { class: "token-list-item__rank", "#{rank}" }
                if let Some(icon) = local_token_icon(&token.id) {
                    img { src: icon, alt: "{logo_alt}" }
                } else {
                    img { src: "{token.image}", alt: "{logo_alt}" }
                }
                div {
                    strong {
                        a {
                            href: "{coingecko_url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "{token.name}"
                        }
                    }
                    span {
                        a {
                            href: "{coingecko_url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "{token.symbol.to_uppercase()}"
                        }
                    }
                }
            }
            span { class: "token-list-item__price", "{format_currency(token.current_price)}" }
            span { class: change_class, "{format_percent(price_change)}" }
            span { "{format_optional_currency(token.market_cap)}" }
            span { "{format_optional_currency(token.total_volume)}" }
        }
    }
}
