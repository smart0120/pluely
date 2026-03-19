# Bundled OpenAI key (compile-time)

You can embed a single **OpenAI API key** at build time. The app will default to:

- **AI:** built-in `openai` provider  
- **Model:** `gpt-5.3-chat-latest` (override with `OPENAI_CHAT_MODEL`)  
- **STT:** built-in `openai-whisper`  
- **Whisper model:** `whisper-1` (override with `OPENAI_WHISPER_MODEL`)

The key is **not** written to localStorage and Dev Space shows a placeholder instead of the secret.

> **Security:** The key still exists inside the binary and can be extracted. Treat this as obfuscation for casual users, not strong protection.

## How to build

1. Create or edit `src-tauri/.env` (loaded automatically from the crate directory):

   ```env
   OPENAI_API_KEY=sk-...
   ```

   Optional:

   ```env
   OPENAI_CHAT_MODEL=gpt-5.3-chat-latest
   OPENAI_WHISPER_MODEL=whisper-1
   ```

2. Or set the variable in your shell before building:

   ```bash
   export OPENAI_API_KEY=sk-...
   npm run tauri build
   ```

3. **Do not commit** `src-tauri/.env` (it is gitignored).

Build logs may contain the key if your tooling echoes environment variables—use a local build or a secret-aware CI step.
