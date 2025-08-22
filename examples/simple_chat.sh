#!/usr/bin/env bash
set -euo pipefail
cargo run -p polyglot -- chat \
  --provider openai \
  --model gpt-4o-mini \
  --prompt "Explain Tokio streams in one paragraph" 
