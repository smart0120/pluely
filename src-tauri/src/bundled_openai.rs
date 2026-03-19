//! Optional OpenAI API key and defaults baked in at compile time.
//! Set `OPENAI_API_KEY` when running `cargo build` / `tauri build` (e.g. in `src-tauri/.env`).
//! The key is embedded in the binary — it is not secret from reverse engineering.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundledOpenaiConfig {
    /// True when a non-empty key was set at build time
    pub enabled: bool,
    pub api_key: String,
    pub chat_model: String,
    pub whisper_model: String,
}

/// Returns embedded OpenAI credentials when `OPENAI_API_KEY` was provided at build time.
#[tauri::command]
pub fn get_bundled_openai_config() -> BundledOpenaiConfig {
    let key = option_env!("BUNDLED_OPENAI_API_KEY").unwrap_or("").trim();
    if key.is_empty() {
        return BundledOpenaiConfig {
            enabled: false,
            api_key: String::new(),
            chat_model: String::new(),
            whisper_model: String::new(),
        };
    }

    BundledOpenaiConfig {
        enabled: true,
        api_key: key.to_string(),
        chat_model: option_env!("BUNDLED_OPENAI_CHAT_MODEL")
            .filter(|s| !s.is_empty())
            .unwrap_or("gpt-5.3-chat-latest")
            .to_string(),
        whisper_model: option_env!("BUNDLED_OPENAI_WHISPER_MODEL")
            .filter(|s| !s.is_empty())
            .unwrap_or("whisper-1")
            .to_string(),
    }
}
