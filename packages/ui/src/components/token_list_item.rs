use dioxus::prelude::*;
use dioxus_charts::LineChart;
use dioxus_i18n::t;

use crate::components::main::{
    format_currency, format_optional_currency, format_percent, local_token_icon,
};
use crate::models::Token;

const SPARKLINE_DAYS: usize = 7;
const SPARKLINE_POINT_COUNT: usize = 50;

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
    let sparkline = token
        .sparkline_in_7d
        .as_ref()
        .map(|sparkline| sparkline.price.clone())
        .unwrap_or_default();
    let sparkline_change = sparkline_change(&sparkline);
    let sparkline_class = if sparkline_change.unwrap_or_default() >= 0.0 {
        "token-sparkline token-sparkline--up"
    } else {
        "token-sparkline token-sparkline--down"
    };
    let sparkline_label = sparkline_change
        .map(format_percent)
        .unwrap_or_else(|| t!("not-available"));
    let sparkline_series = downsample_sparkline(&sparkline);
    let sparkline_labels = (0..sparkline_series.len())
        .map(|_| String::new())
        .collect::<Vec<_>>();

    rsx! {
        li { class: "token-list-item",
            div { class: "token-list-item__asset",
                span { class: "token-list-item__rank", "#{rank}" }
                if let Some(icon) = local_token_icon(&token.id) {
                    img {
                        src: icon,
                        width: "32",
                        height: "32",
                        alt: "{logo_alt}",
                    }
                } else {
                    img {
                        src: "{token.image}",
                        width: "32",
                        height: "32",
                        alt: "{logo_alt}",
                    }
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
            div {
                class: "{sparkline_class}",
                title: "{SPARKLINE_DAYS}D price movement: {sparkline_label}",
                aria_label: "{SPARKLINE_DAYS}D price movement: {sparkline_label}",
                if sparkline_series.len() > 1 {
                    LineChart {
                        width: "92px",
                        height: "30px",
                        viewbox_width: 92,
                        viewbox_height: 30,
                        padding_top: 3,
                        padding_bottom: 3,
                        show_grid: false,
                        show_labels: false,
                        show_dots: false,
                        show_line_labels: false,
                        line_width: "5",
                        class_chart_line: "token-sparkline__chart",
                        class_line_path: "token-sparkline__path",
                        labels: sparkline_labels,
                        series: vec![sparkline_series],
                    }
                } else {
                    span { class: "token-sparkline__empty", {t!("not-available")} }
                }
            }
        }
    }
}

fn sparkline_change(prices: &[f64]) -> Option<f64> {
    let first = prices.iter().copied().find(|value| value.is_finite())?;
    let last = prices
        .iter()
        .rev()
        .copied()
        .find(|value| value.is_finite())?;

    if first == 0.0 {
        None
    } else {
        Some(((last - first) / first) * 100.0)
    }
}

fn downsample_sparkline(prices: &[f64]) -> Vec<f32> {
    let prices = prices
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .collect::<Vec<_>>();

    if prices.len() <= SPARKLINE_POINT_COUNT {
        return prices.into_iter().map(|value| value as f32).collect();
    }

    let last_index = prices.len() - 1;
    let last_slot = SPARKLINE_POINT_COUNT - 1;

    (0..SPARKLINE_POINT_COUNT)
        .map(|slot| {
            let source_index = ((slot * last_index) + (last_slot / 2)) / last_slot;
            prices[source_index] as f32
        })
        .collect()
}
