use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::toast::{wait_for_toast_timeout, Toast, ToastTone};
use crate::components::token_list::TokenList;
use crate::models::{TokenLoadResult, TokenSource};
use crate::services::token_service::{
    load_tokens, refresh_tokens_from_online, token_result_has_sparkline_data,
};

const TOKEN_LIST_CSS: Asset = asset!("/assets/styling/token_list.css");
const BITCOIN_HERO_IMAGE: Asset = asset!("/assets/images/crypto/bitcoin-hero-slick.png");

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TokenLoadRequest {
    pub sequence: u64,
}

impl TokenLoadRequest {
    pub fn initial() -> Self {
        Self { sequence: 0 }
    }
}

#[component]
pub fn Main() -> Element {
    let token_load_request = use_context::<Signal<TokenLoadRequest>>();
    let mut token_load_cache = use_context::<Signal<Option<Result<TokenLoadResult, String>>>>();
    let initial_request_sequence = use_signal(|| token_load_request().sequence);
    let mut toast = use_context::<Signal<Option<Toast>>>();
    let mut toast_sequence = use_signal(|| 0_u64);
    let mut last_toast_key = use_signal(|| None::<String>);
    let mut tokens = use_resource(move || async move {
        let request = token_load_request();
        let initial_request_sequence = initial_request_sequence();

        if request.sequence > initial_request_sequence {
            refresh_tokens_from_online().await
        } else if let Some(cached) = token_load_cache.peek().clone() {
            match cached {
                Ok(result) if !token_result_has_sparkline_data(&result) => load_tokens().await,
                _ => cached,
            }
        } else {
            load_tokens().await
        }
    });

    use_effect(move || {
        let request = token_load_request();
        let initial_request_sequence = initial_request_sequence();

        if request.sequence > initial_request_sequence {
            token_load_cache.set(None);
            tokens.clear();
        }
    });

    let token_result = tokens();

    use_effect(move || {
        if let Some(result) = tokens() {
            token_load_cache.set(Some(result));
        }
    });

    use_effect(move || {
        let request = token_load_request();
        let toast_details = match tokens() {
            Some(Ok(result)) => {
                let source = result.source.clone();
                let (message, tone) = token_source_toast(&source);
                (
                    format!("success:{}:{source:?}", request.sequence),
                    message,
                    tone,
                )
            }
            Some(Err(message)) => (
                format!("error:{}:{message}", request.sequence),
                t!("market-data-unavailable"),
                ToastTone::Error,
            ),
            None => (
                format!("loading:{}", request.sequence),
                t!("loading-market-data"),
                ToastTone::Info,
            ),
        };

        if last_toast_key.peek().as_ref() == Some(&toast_details.0) {
            return;
        }

        let next_id = *toast_sequence.peek() + 1;
        toast_sequence.set(next_id);
        last_toast_key.set(Some(toast_details.0));
        toast.set(Some(Toast {
            id: next_id,
            message: toast_details.1,
            tone: toast_details.2,
        }));
    });

    use_effect(move || {
        if let Some(active_toast) = toast() {
            spawn(async move {
                wait_for_toast_timeout().await;

                if toast.peek().as_ref().map(|toast| toast.id) == Some(active_toast.id) {
                    toast.set(None);
                }
            });
        }
    });

    let online_last_updated_text = token_result
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .map(|result| {
            let updated_at = result
                .online_last_updated_at
                .map(format_relative_timestamp)
                .unwrap_or_else(|| t!("not-loaded-this-session"));

            format!("{updated_at} ({})", token_source_label(&result.source))
        })
        .unwrap_or_else(|| t!("not-loaded-this-session"));

    rsx! {
        document::Link { rel: "stylesheet", href: TOKEN_LIST_CSS }

        main { class: "token-page",
            section { class: "token-page__intro",
                img {
                    class: "token-page__bitcoin-art",
                    src: BITCOIN_HERO_IMAGE,
                    width: "1120",
                    height: "320",
                    alt: "",
                }
                div { class: "token-page__intro-copy",
                    h1 { {t!("top-crypto-currencies")} }
                    p { {t!("market-data-provider")} }
                    div { class: "token-page__timestamps",
                        p { {t!("online-last-updated", value: online_last_updated_text)} }
                    }
                }
            }

            match token_result {
                Some(Ok(TokenLoadResult { tokens, .. })) => rsx! {
                    TokenList { tokens }
                },
                Some(Err(message)) => rsx! {
                    div { class: "token-state token-state--error",
                        h2 { {t!("market-data-unavailable-heading")} }
                        p { "{message}" }
                    }
                },
                None => rsx! {},
            }
        }
    }
}

fn token_source_toast(source: &TokenSource) -> (String, ToastTone) {
    match source {
        TokenSource::BrowserSnapshot | TokenSource::Database => {
            (token_source_label(source), ToastTone::Info)
        }
        TokenSource::Online => (token_source_label(source), ToastTone::Success),
    }
}

fn token_source_label(source: &TokenSource) -> String {
    match source {
        TokenSource::BrowserSnapshot | TokenSource::Database => t!("source-database"),
        TokenSource::Online => t!("source-online"),
    }
}

fn format_relative_timestamp(value: chrono::DateTime<chrono::Utc>) -> String {
    let elapsed = chrono::Utc::now().signed_duration_since(value);

    if elapsed.num_seconds() < 1 {
        t!("just-now")
    } else if elapsed.num_seconds() < 60 {
        t!("duration-seconds", value: elapsed.num_seconds())
    } else if elapsed.num_minutes() < 60 {
        t!("duration-minutes", value: elapsed.num_minutes())
    } else if elapsed.num_hours() < 24 {
        t!("duration-hours", value: elapsed.num_hours())
    } else {
        t!("duration-days", value: elapsed.num_days())
    }
}

pub(crate) fn format_optional_currency(value: Option<f64>) -> String {
    value
        .map(format_compact_currency)
        .unwrap_or_else(|| t!("not-available"))
}

pub(crate) fn format_currency(value: f64) -> String {
    if value >= 1.0 {
        format!("${value:.2}")
    } else {
        format!("${value:.6}")
    }
}

pub(crate) fn format_compact_currency(value: f64) -> String {
    let absolute = value.abs();

    if absolute >= 1_000_000_000_000.0 {
        format!("${:.2}T", value / 1_000_000_000_000.0)
    } else if absolute >= 1_000_000_000.0 {
        format!("${:.2}B", value / 1_000_000_000.0)
    } else if absolute >= 1_000_000.0 {
        format!("${:.2}M", value / 1_000_000_000.0)
    } else {
        format_currency(value)
    }
}

pub(crate) fn format_percent(value: f64) -> String {
    if value >= 0.0 {
        format!("+{value:.2}%")
    } else {
        format!("{value:.2}%")
    }
}

pub(crate) fn local_token_icon(id: &str) -> Option<Asset> {
    match id {
        "bitcoin" => Some(asset!("/assets/images/tokens/btc.png")),
        "ethereum" => Some(asset!("/assets/images/tokens/eth.png")),
        "tether" => Some(asset!("/assets/images/tokens/usdt.png")),
        "ripple" => Some(asset!("/assets/images/tokens/xrp.png")),
        "binancecoin" => Some(asset!("/assets/images/tokens/bnb.png")),
        "usd-coin" => Some(asset!("/assets/images/tokens/usdc.png")),
        "solana" => Some(asset!("/assets/images/tokens/sol.png")),
        "tron" => Some(asset!("/assets/images/tokens/trx.png")),
        "figure-heloc" => Some(asset!("/assets/images/tokens/figr_heloc.png")),
        "dogecoin" => Some(asset!("/assets/images/tokens/doge.png")),
        "whitebit" => Some(asset!("/assets/images/tokens/wbt.png")),
        "usds" => Some(asset!("/assets/images/tokens/usds.webp")),
        "leo-token" => Some(asset!("/assets/images/tokens/leo.png")),
        "hyperliquid" => Some(asset!("/assets/images/tokens/hype.jpg")),
        "cardano" => Some(asset!("/assets/images/tokens/ada.png")),
        "bitcoin-cash" => Some(asset!("/assets/images/tokens/bch.png")),
        "monero" => Some(asset!("/assets/images/tokens/xmr.png")),
        "chainlink" => Some(asset!("/assets/images/tokens/link.png")),
        "canton-network" => Some(asset!("/assets/images/tokens/cc.png")),
        "zcash" => Some(asset!("/assets/images/tokens/zec.png")),
        _ => None,
    }
}
