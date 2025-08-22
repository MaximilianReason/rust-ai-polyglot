#!/usr/bin/env bash
set -euo pipefail

PROMPT="${1:-Summarize Singapore hawker culture in 3 bullets}"
echo "OpenAI:"
cargo run -p polyglot -- chat --provider openai --model gpt-4.1-mini --prompt "$PROMPT"
echo
echo "OpenRouter (Llama 3 family or your choice):"
cargo run -p polyglot -- chat --provider openrouter --model "meta-llama/llama-3.1-70b-instruct" --prompt "$PROMPT"
