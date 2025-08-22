use clap::{Parser, Subcommand, ValueEnum};
use llm_core::{providers, types::*, Provider};
use anyhow::Result;
use dotenv::dotenv;

#[derive(Clone, Debug, ValueEnum)]
enum ProviderArg { Openai, Azure, Anthropic, Openrouter, Hf, Replicate, Gemini }

#[derive(Parser)]
#[command(name="polyglot", version, about="Multi-provider AI CLI")]
struct Args {
    #[command(subcommand)]
    cmd: Cmd
}

#[derive(Subcommand)]
enum Cmd {
    /// Single-turn chat (use --stream to stream tokens when supported)
    Chat {
        #[arg(long, value_enum)] provider: ProviderArg,
        #[arg(long)] model: String,
        #[arg(long)] prompt: String,
        #[arg(long, default_value_t=false)] stream: bool
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let args = Args::parse();

    match args.cmd {
        Cmd::Chat { provider, model, prompt, stream } => {
            let sys = ChatMessage::system("You are a concise assistant.");
            let user = ChatMessage::user(prompt);
            let msgs = vec![sys, user];

            let kind = match provider {
                ProviderArg::Openai => ProviderKind::OpenAI,
                ProviderArg::Azure => ProviderKind::Azure,
                ProviderArg::Anthropic => ProviderKind::Anthropic,
                ProviderArg::Openrouter => ProviderKind::OpenRouter,
                ProviderArg::Hf => ProviderKind::HuggingFace,
                ProviderArg::Replicate => ProviderKind::Replicate,
                ProviderArg::Gemini => ProviderKind::Gemini,
            };

            let p = providers::make_provider(kind)?;
            if stream {
                p.stream_chat(&msgs, &model).await?;
                println!();
            } else {
                let out = p.chat(&msgs, &model).await?;
                println!("{out}");
            }
        }
    }

    Ok(())
}
