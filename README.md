# polyglot-ai

A multi-provider **AI CLI & Rust SDK** that unifies chat, streaming, embeddings, and image generation across **OpenAI/Azure**, **Anthropic**, **OpenRouter**, **Hugging Face Inference API**, **Replicate**, and **Gemini**—all behind a single trait.

## Why
- Swap providers for cost, latency, or compliance—no app changes.
- One CLI for local experiments; one library for integration.

## Quick start
```bash
git clone https://github.com/youruser/polyglot-ai
cd polyglot-ai
cp .env.example .env        # fill your keys
cargo build                 # default features: openai + openrouter

## Example
cargo run -p polyglot -- chat \
  --provider openai \
  --model gpt-4.1-mini \
  --prompt "Summarize Rust ownership in 2 bullets"

## Enabling providers at compile time:
## Add Anthropic + HuggingFace
cargo build -p polyglot -F "llm_core/provider-anthropic llm_core/provider-hf"

## How To Use
# 1) Initialize repo
git init polyglot-ai && cd polyglot-ai
# Paste the files above into this structure (or I can generate a zip on request)

# 2) Install deps & build
cp .env.example .env  # fill your keys
cargo build

# 3) Run
cargo run -p polyglot -- chat --provider openai --model gpt-4.1-mini --prompt "Hello!"

# 4) Enabling more providers
# Anthropic + HF at compile time:
cargo run -p polyglot -F "llm_core/provider-anthropic llm_core/provider-hf" -- \
  chat --provider anthropic --model claude-3-5-sonnet-latest --prompt "Hi!"

