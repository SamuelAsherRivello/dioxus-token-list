use chrono::{DateTime, Utc};

use crate::client::models::{Token, TokenLoadResult, TokenSource};
use crate::client::services::{database_service, online_service};

pub async fn load_tokens() -> Result<TokenLoadResult, String> {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(result) = crate::client::services::storage_service::load_token_snapshot() {
            if token_result_has_sparkline_data(&result) {
                return Ok(result);
            }
        }
    }

    if let Ok(Some(result)) = load_tokens_from_database().await {
        if token_result_has_sparkline_data(&result) {
            save_browser_snapshot(&result);
            return Ok(result);
        }
    }

    let result = refresh_tokens_from_online().await?;
    save_browser_snapshot(&result);

    Ok(result)
}

pub async fn refresh_tokens_from_online() -> Result<TokenLoadResult, String> {
    let tokens = online_service::fetch_top_tokens().await?;
    let online_last_updated_at = Some(Utc::now());

    let db_last_loaded_at =
        database_service::replace_cached_tokens(tokens.clone(), online_last_updated_at)
            .await
            .ok()
            .map(|_| Utc::now());

    let result = TokenLoadResult {
        tokens,
        source: TokenSource::Online,
        online_last_updated_at,
        db_last_loaded_at,
    };

    save_browser_snapshot(&result);

    Ok(result)
}

async fn load_tokens_from_database() -> Result<Option<TokenLoadResult>, String> {
    let cached = database_service::load_cached_tokens()
        .await
        .map_err(|error| error.to_string())?;

    if cached.tokens.is_empty() {
        return Ok(None);
    }

    Ok(Some(TokenLoadResult {
        tokens: cached.tokens,
        source: TokenSource::Database,
        online_last_updated_at: cached.online_last_updated_at,
        db_last_loaded_at: Some(DateTime::<Utc>::from(std::time::SystemTime::now())),
    }))
}

fn save_browser_snapshot(result: &TokenLoadResult) {
    #[cfg(target_arch = "wasm32")]
    crate::client::services::storage_service::save_token_snapshot(result);

    #[cfg(not(target_arch = "wasm32"))]
    let _ = result;
}

pub fn token_result_has_sparkline_data(result: &TokenLoadResult) -> bool {
    tokens_have_sparkline_data(&result.tokens)
}

fn tokens_have_sparkline_data(tokens: &[Token]) -> bool {
    !tokens.is_empty()
        && tokens.iter().all(|token| {
            token.sparkline_in_7d.as_ref().is_some_and(|sparkline| {
                sparkline
                    .price
                    .iter()
                    .filter(|price| price.is_finite())
                    .take(2)
                    .count()
                    >= 2
            })
        })
}
