use reqwest::header;

use crate::client::models::Token;

const COINGECKO_MARKETS_URL: &str = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=20&page=1&sparkline=true&price_change_percentage=24h";
const COINGECKO_TIMEOUT_SECONDS: u64 = 5;

pub async fn fetch_top_tokens() -> Result<Vec<Token>, String> {
    fetch_top_tokens_with_timeout().await
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_top_tokens_with_timeout() -> Result<Vec<Token>, String> {
    let client = reqwest::Client::builder()
        .user_agent("dioxus-token-list/0.1")
        .timeout(std::time::Duration::from_secs(COINGECKO_TIMEOUT_SECONDS))
        .build()
        .map_err(|error| format!("Could not build request client: {error}"))?;

    fetch_with_client(client).await
}

#[cfg(target_arch = "wasm32")]
async fn fetch_top_tokens_with_timeout() -> Result<Vec<Token>, String> {
    use futures::future::{select, Either};

    let client = reqwest::Client::builder()
        .build()
        .map_err(|error| format!("Could not build request client: {error}"))?;
    let request = fetch_with_client(client);
    let timeout = async {
        gloo_timers::future::TimeoutFuture::new((COINGECKO_TIMEOUT_SECONDS * 1_000) as u32).await;
        Err("CoinGecko request timed out".to_string())
    };

    futures::pin_mut!(request);
    futures::pin_mut!(timeout);

    match select(request, timeout).await {
        Either::Left((result, _)) => result,
        Either::Right((result, _)) => result,
    }
}

async fn fetch_with_client(client: reqwest::Client) -> Result<Vec<Token>, String> {
    client
        .get(COINGECKO_MARKETS_URL)
        .header(header::ACCEPT, "application/json")
        .send()
        .await
        .map_err(|error| format!("Could not reach CoinGecko: {error}"))?
        .error_for_status()
        .map_err(|error| format!("CoinGecko returned an error: {error}"))?
        .json::<Vec<Token>>()
        .await
        .map_err(|error| format!("Could not read market data: {error}"))
}
