use axum::{extract::State, http::StatusCode};
pub async fn hello() -> Result<String, StatusCode> {
    // Ok(format!(
    //     "\nhello world! Using configuration from {}\n\n",
    //     state
    //         .settings
    //         .load()
    //         .config_info
    //         .location
    //         .clone()
    //         .unwrap_or("missing config location".to_string())
    // )
    // .to_string())

    Ok("hello world!".to_string())
}
