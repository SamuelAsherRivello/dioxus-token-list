use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::client::models::Token;

#[cfg(not(target_arch = "wasm32"))]
const ONLINE_LAST_UPDATED_KEY: &str = "online_last_updated_at";

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CachedTokens {
    pub tokens: Vec<Token>,
    pub online_last_updated_at: Option<DateTime<Utc>>,
}

pub async fn load_cached_tokens() -> Result<CachedTokens, String> {
    platform::load_cached_tokens().await
}

pub async fn replace_cached_tokens(
    tokens: Vec<Token>,
    online_last_updated_at: Option<DateTime<Utc>>,
) -> Result<(), String> {
    platform::replace_cached_tokens(&tokens, online_last_updated_at).await
}

#[cfg(target_arch = "wasm32")]
mod platform {
    use chrono::{DateTime, Utc};

    use crate::client::models::Token;
    use crate::client::services::database_service::CachedTokens;

    pub async fn load_cached_tokens() -> Result<CachedTokens, String> {
        Ok(CachedTokens {
            tokens: Vec::new(),
            online_last_updated_at: None,
        })
    }

    pub async fn replace_cached_tokens(
        _tokens: &[Token],
        _online_last_updated_at: Option<DateTime<Utc>>,
    ) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    use std::fs;
    use std::path::PathBuf;

    use chrono::{DateTime, Utc};
    use rusqlite::{params, Connection, OptionalExtension};

    use crate::client::models::Token;
    use crate::client::services::database_service::{CachedTokens, ONLINE_LAST_UPDATED_KEY};

    pub async fn load_cached_tokens() -> Result<CachedTokens, String> {
        let connection = open_connection()?;
        migrate(&connection)?;

        let mut statement = connection
            .prepare(
                "SELECT id, symbol, name, image, current_price, market_cap, market_cap_rank,
                    total_volume, price_change_percentage_24h, price_sparkline_7d
             FROM token_list_items
             ORDER BY COALESCE(market_cap_rank, 999999), name",
            )
            .map_err(|error| format!("Could not read token cache: {error}"))?;

        let rows = statement
            .query_map([], |row| {
                Ok(Token {
                    id: row.get(0)?,
                    symbol: row.get(1)?,
                    name: row.get(2)?,
                    image: row.get(3)?,
                    current_price: row.get(4)?,
                    market_cap: row.get(5)?,
                    market_cap_rank: row.get(6)?,
                    total_volume: row.get(7)?,
                    price_change_percentage_24h: row.get(8)?,
                    sparkline_in_7d: deserialize_sparkline(row.get(9)?),
                })
            })
            .map_err(|error| format!("Could not read token cache: {error}"))?;

        let tokens = rows
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("Could not read token cache row: {error}"))?;

        let online_last_updated_at = read_metadata(&connection, ONLINE_LAST_UPDATED_KEY)?
            .and_then(|value| DateTime::parse_from_rfc3339(&value).ok())
            .map(|value| value.with_timezone(&Utc));

        Ok(CachedTokens {
            tokens,
            online_last_updated_at,
        })
    }

    pub async fn replace_cached_tokens(
        tokens: &[Token],
        online_last_updated_at: Option<DateTime<Utc>>,
    ) -> Result<(), String> {
        let mut connection = open_connection()?;
        migrate(&connection)?;

        let transaction = connection
            .transaction()
            .map_err(|error| format!("Could not update token cache: {error}"))?;

        transaction
            .execute("DELETE FROM token_list_items", [])
            .map_err(|error| format!("Could not clear token cache: {error}"))?;

        for token in tokens {
            transaction
                .execute(
                    "INSERT INTO token_list_items (
                    id, symbol, name, image, current_price, market_cap, market_cap_rank,
                    total_volume, price_change_percentage_24h, price_sparkline_7d
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                    params![
                        token.id,
                        token.symbol,
                        token.name,
                        token.image,
                        token.current_price,
                        token.market_cap,
                        token.market_cap_rank,
                        token.total_volume,
                        token.price_change_percentage_24h,
                        serialize_sparkline(token),
                    ],
                )
                .map_err(|error| format!("Could not write token cache row: {error}"))?;
        }

        if let Some(updated_at) = online_last_updated_at {
            transaction
                .execute(
                    "INSERT INTO token_cache_metadata (key, value)
                 VALUES (?1, ?2)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value",
                    params![ONLINE_LAST_UPDATED_KEY, updated_at.to_rfc3339()],
                )
                .map_err(|error| format!("Could not write token cache metadata: {error}"))?;
        }

        transaction
            .commit()
            .map_err(|error| format!("Could not save token cache: {error}"))
    }

    fn open_connection() -> Result<Connection, String> {
        let path = database_path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("Could not create database directory: {error}"))?;
        }

        Connection::open(path).map_err(|error| format!("Could not open token database: {error}"))
    }

    fn database_path() -> Result<PathBuf, String> {
        let root = std::env::current_dir()
            .map_err(|error| format!("Could not find current directory for database: {error}"))?;

        Ok(root.join("data").join("token-list.sqlite"))
    }

    fn migrate(connection: &Connection) -> Result<(), String> {
        connection
            .execute_batch(
                "
            CREATE TABLE IF NOT EXISTS token_list_items (
                id TEXT PRIMARY KEY,
                symbol TEXT NOT NULL,
                name TEXT NOT NULL,
                image TEXT NOT NULL,
                current_price REAL NOT NULL,
                market_cap REAL,
                market_cap_rank INTEGER,
                total_volume REAL,
                price_change_percentage_24h REAL,
                price_sparkline_7d TEXT
            );

            CREATE TABLE IF NOT EXISTS token_cache_metadata (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            ",
            )
            .map_err(|error| format!("Could not migrate token database: {error}"))?;

        if !column_exists(connection, "token_list_items", "price_sparkline_7d")? {
            connection
                .execute(
                    "ALTER TABLE token_list_items ADD COLUMN price_sparkline_7d TEXT",
                    [],
                )
                .map_err(|error| format!("Could not migrate token sparkline cache: {error}"))?;
        }

        Ok(())
    }

    fn read_metadata(connection: &Connection, key: &str) -> Result<Option<String>, String> {
        connection
            .query_row(
                "SELECT value FROM token_cache_metadata WHERE key = ?1",
                params![key],
                |row| row.get(0),
            )
            .optional()
            .map_err(|error| format!("Could not read token cache metadata: {error}"))
    }

    fn column_exists(connection: &Connection, table: &str, column: &str) -> Result<bool, String> {
        let mut statement = connection
            .prepare(&format!("PRAGMA table_info({table})"))
            .map_err(|error| format!("Could not inspect token cache schema: {error}"))?;
        let columns = statement
            .query_map([], |row| row.get::<_, String>(1))
            .map_err(|error| format!("Could not inspect token cache schema: {error}"))?;

        for name in columns {
            if name.map_err(|error| format!("Could not read token cache schema: {error}"))?
                == column
            {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn serialize_sparkline(token: &Token) -> Option<String> {
        token
            .sparkline_in_7d
            .as_ref()
            .and_then(|sparkline| serde_json::to_string(sparkline).ok())
    }

    fn deserialize_sparkline(
        value: Option<String>,
    ) -> Option<crate::client::models::TokenSparkline> {
        value.and_then(|value| serde_json::from_str(&value).ok())
    }
}
