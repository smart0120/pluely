fn main() {
    // Load .env from the crate directory (src-tauri) so it works when building from repo root
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let env_path = std::path::PathBuf::from(&manifest_dir).join(".env");
    dotenv::from_path(&env_path).ok();
    dotenv::dotenv().ok();

    // Optional: embed OpenAI key for production builds (never echo the value)
    if let Ok(k) = std::env::var("OPENAI_API_KEY") {
        if !k.trim().is_empty() {
            println!("cargo:rustc-env=BUNDLED_OPENAI_API_KEY={}", k);
        }
    }
    if let Ok(m) = std::env::var("OPENAI_CHAT_MODEL") {
        if !m.trim().is_empty() {
            println!("cargo:rustc-env=BUNDLED_OPENAI_CHAT_MODEL={}", m);
        }
    }
    if let Ok(m) = std::env::var("OPENAI_WHISPER_MODEL") {
        if !m.trim().is_empty() {
            println!("cargo:rustc-env=BUNDLED_OPENAI_WHISPER_MODEL={}", m);
        }
    }

    if let Ok(payment_endpoint) = std::env::var("PAYMENT_ENDPOINT") {
        println!("cargo:rustc-env=PAYMENT_ENDPOINT={}", payment_endpoint);
    }

    if let Ok(api_access_key) = std::env::var("API_ACCESS_KEY") {
        println!("cargo:rustc-env=API_ACCESS_KEY={}", api_access_key);
    }

    if let Ok(app_endpoint) = std::env::var("APP_ENDPOINT") {
        println!("cargo:rustc-env=APP_ENDPOINT={}", app_endpoint);
    }

    if let Ok(posthog_api_key) = std::env::var("POSTHOG_API_KEY") {
        println!("cargo:rustc-env=POSTHOG_API_KEY={}", posthog_api_key);
    }

    tauri_build::build()
}
