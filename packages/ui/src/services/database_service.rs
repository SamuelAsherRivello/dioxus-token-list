use chrono::{DateTime, Utc};

use crate::models::Token;

const ONLINE_LAST_UPDATED_KEY: &str = "online_last_updated_at";

#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub struct CachedTokens {
    pub tokens: Vec<Token>,
    pub online_last_updated_at: Option<DateTime<Utc>>,
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use std::fs;
    use std::path::PathBuf;

    use chrono::{DateTime, Utc};
    use rusqlite::{params, Connection, OptionalExtension};

    use crate::models::Token;
    use crate::services::database_service::{CachedTokens, ONLINE_LAST_UPDATED_KEY};

    pub async fn load_cached_tokens() -> Result<CachedTokens, String> {
        let connection = open_connection()?;
        migrate(&connection)?;

        let mut statement = connection
            .prepare(
                "SELECT id, symbol, name, image, current_price, market_cap, market_cap_rank,
                    total_volume, price_change_percentage_24h
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
                    total_volume, price_change_percentage_24h
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
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
                price_change_percentage_24h REAL
            );

            CREATE TABLE IF NOT EXISTS token_cache_metadata (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            ",
            )
            .map_err(|error| format!("Could not migrate token database: {error}"))
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
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
mod browser {
    use std::future::Future;

    use chrono::{DateTime, Utc};
    use futures::future::{select, Either};
    use js_sys::{Array, Reflect};
    use wasm_bindgen::{JsCast, JsValue};

    use crate::models::Token;
    use crate::services::database_service::{CachedTokens, ONLINE_LAST_UPDATED_KEY};

    const DATABASE_NAME: &str = "token-list.sqlite3";
    const SQLITE_WORKER_PATH: &str = "assets/sqlite.org/sqlite3-worker1.js?sqlite3.dir=.";
    const SQLITE_INIT_TIMEOUT_MS: u32 = 2_000;
    const SQLITE_OPEN_TIMEOUT_MS: u32 = 500;
    const SQLITE_OPERATION_TIMEOUT_MS: u32 = 2_000;
    const SQLITE_OPEN_ATTEMPTS: usize = 4;

    pub async fn load_cached_tokens() -> Result<CachedTokens, String> {
        initialize_database().await?;

        let result = sqlite_query(
            "SELECT id, symbol, name, image, current_price, market_cap, market_cap_rank,
                    total_volume, price_change_percentage_24h
             FROM token_list_items
             ORDER BY COALESCE(market_cap_rank, 999999), name",
            vec![],
        )
        .await?;

        let tokens = result_rows(&result)?
            .iter()
            .map(token_from_row)
            .collect::<Result<Vec<_>, _>>()?;

        let online_last_updated_at = read_metadata(ONLINE_LAST_UPDATED_KEY)
            .await?
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
        initialize_database().await?;

        sqlite_exec("BEGIN IMMEDIATE", vec![]).await?;

        let result = replace_cached_tokens_in_transaction(tokens, online_last_updated_at).await;

        if result.is_ok() {
            sqlite_exec("COMMIT", vec![]).await?;
        } else {
            let _ = sqlite_exec("ROLLBACK", vec![]).await;
        }

        result
    }

    async fn replace_cached_tokens_in_transaction(
        tokens: &[Token],
        online_last_updated_at: Option<DateTime<Utc>>,
    ) -> Result<(), String> {
        sqlite_exec("DELETE FROM token_list_items", vec![]).await?;

        for token in tokens {
            sqlite_exec(
                "INSERT INTO token_list_items (
                    id, symbol, name, image, current_price, market_cap, market_cap_rank,
                    total_volume, price_change_percentage_24h
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                vec![
                    JsValue::from_str(&token.id),
                    JsValue::from_str(&token.symbol),
                    JsValue::from_str(&token.name),
                    JsValue::from_str(&token.image),
                    JsValue::from_f64(token.current_price),
                    optional_f64(token.market_cap),
                    token
                        .market_cap_rank
                        .map(|value| JsValue::from_f64(value as f64))
                        .unwrap_or(JsValue::NULL),
                    optional_f64(token.total_volume),
                    optional_f64(token.price_change_percentage_24h),
                ],
            )
            .await?;
        }

        if let Some(updated_at) = online_last_updated_at {
            sqlite_exec(
                "INSERT INTO token_cache_metadata (key, value)
                 VALUES (?, ?)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value",
                vec![
                    JsValue::from_str(ONLINE_LAST_UPDATED_KEY),
                    JsValue::from_str(&updated_at.to_rfc3339()),
                ],
            )
            .await?;
        }

        Ok(())
    }

    async fn initialize_database() -> Result<(), String> {
        match with_timeout(
            sqlite_wasm::autostart(SQLITE_WORKER_PATH),
            SQLITE_INIT_TIMEOUT_MS,
            "Browser SQLite worker initialization timed out",
        )
        .await
        {
            Ok(_) => {}
            Err(error) => {
                let message = js_error(error);

                if !message.contains("Worker initialization timeout")
                    && !message.contains("worker initialization timed out")
                {
                    return Err(message);
                }
            }
        }

        if !sqlite_wasm::is_open() {
            open_database_with_retry().await?;
        }

        migrate().await
    }

    async fn open_database_with_retry() -> Result<(), String> {
        let mut last_error = None;

        for _ in 0..SQLITE_OPEN_ATTEMPTS {
            match with_timeout(
                sqlite_wasm::open(DATABASE_NAME),
                SQLITE_OPEN_TIMEOUT_MS,
                "Browser SQLite open timed out",
            )
            .await
            {
                Ok(_) => return Ok(()),
                Err(error) => {
                    last_error = Some(js_error(error));
                    gloo_timers::future::TimeoutFuture::new(250).await;
                }
            }
        }

        Err(last_error.unwrap_or_else(|| "Could not open browser SQLite database".to_string()))
    }

    async fn migrate() -> Result<(), String> {
        sqlite_exec(
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
                price_change_percentage_24h REAL
            );

            CREATE TABLE IF NOT EXISTS token_cache_metadata (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            ",
            vec![],
        )
        .await
    }

    async fn read_metadata(key: &str) -> Result<Option<String>, String> {
        let result = sqlite_query(
            "SELECT value FROM token_cache_metadata WHERE key = ?",
            vec![JsValue::from_str(key)],
        )
        .await?;

        let rows = result_rows(&result)?;
        let row = rows.get(0);
        if row.is_undefined() {
            return Ok(None);
        }

        Ok(string_field(&row, "value"))
    }

    fn result_rows(value: &JsValue) -> Result<Array, String> {
        let result = Reflect::get(value, &JsValue::from_str("result")).map_err(js_error)?;
        let rows = Reflect::get(&result, &JsValue::from_str("resultRows")).map_err(js_error)?;

        rows.dyn_into::<Array>()
            .map_err(|_| "SQLite returned rows in an unexpected format".to_string())
    }

    fn token_from_row(row: JsValue) -> Result<Token, String> {
        Ok(Token {
            id: required_string_field(&row, "id")?,
            symbol: required_string_field(&row, "symbol")?,
            name: required_string_field(&row, "name")?,
            image: required_string_field(&row, "image")?,
            current_price: required_f64_field(&row, "current_price")?,
            market_cap: f64_field(&row, "market_cap"),
            market_cap_rank: f64_field(&row, "market_cap_rank").map(|value| value as u32),
            total_volume: f64_field(&row, "total_volume"),
            price_change_percentage_24h: f64_field(&row, "price_change_percentage_24h"),
        })
    }

    fn required_string_field(row: &JsValue, name: &str) -> Result<String, String> {
        string_field(row, name).ok_or_else(|| format!("SQLite row is missing {name}"))
    }

    fn string_field(row: &JsValue, name: &str) -> Option<String> {
        let value = Reflect::get(row, &JsValue::from_str(name)).ok()?;

        if value.is_null() || value.is_undefined() {
            None
        } else {
            value.as_string()
        }
    }

    fn required_f64_field(row: &JsValue, name: &str) -> Result<f64, String> {
        f64_field(row, name).ok_or_else(|| format!("SQLite row is missing {name}"))
    }

    fn f64_field(row: &JsValue, name: &str) -> Option<f64> {
        let value = Reflect::get(row, &JsValue::from_str(name)).ok()?;

        if value.is_null() || value.is_undefined() {
            None
        } else {
            value.as_f64()
        }
    }

    fn optional_f64(value: Option<f64>) -> JsValue {
        value.map(JsValue::from_f64).unwrap_or(JsValue::NULL)
    }

    async fn sqlite_exec(sql: &str, args: Vec<JsValue>) -> Result<(), String> {
        with_timeout(
            sqlite_wasm::exec(sql, args),
            SQLITE_OPERATION_TIMEOUT_MS,
            "Browser SQLite exec timed out",
        )
        .await
        .map(|_| ())
        .map_err(js_error)
    }

    async fn sqlite_query(sql: &str, args: Vec<JsValue>) -> Result<JsValue, String> {
        with_timeout(
            sqlite_wasm::query(sql, args),
            SQLITE_OPERATION_TIMEOUT_MS,
            "Browser SQLite query timed out",
        )
        .await
        .map_err(js_error)
    }

    async fn with_timeout<F, T>(
        future: F,
        timeout_ms: u32,
        timeout_message: &'static str,
    ) -> Result<T, JsValue>
    where
        F: Future<Output = Result<T, JsValue>>,
    {
        let timeout = async move {
            gloo_timers::future::TimeoutFuture::new(timeout_ms).await;
            Err(JsValue::from_str(timeout_message))
        };

        futures::pin_mut!(future);
        futures::pin_mut!(timeout);

        match select(future, timeout).await {
            Either::Left((result, _)) => result,
            Either::Right((result, _)) => result,
        }
    }

    fn js_error(value: JsValue) -> String {
        js_error_message(&value)
    }

    fn js_error_message(value: &JsValue) -> String {
        value
            .as_string()
            .unwrap_or_else(|| "Browser SQLite operation failed".to_string())
    }
}

#[cfg(target_arch = "wasm32")]
pub use browser::replace_cached_tokens;

#[cfg(not(target_arch = "wasm32"))]
pub use native::{load_cached_tokens, replace_cached_tokens};
