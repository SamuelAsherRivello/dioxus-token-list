use dioxus::prelude::*;

#[get("/api/server/try")]
pub async fn try_server() -> Result<String, ServerFnError> {
    Ok("Server function is available.".to_string())
}
