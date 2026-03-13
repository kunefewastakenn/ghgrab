use anyhow::Result;
use clap::{Parser, Subcommand};
use ghgrab::config::Config;
use ghgrab::ui;

#[derive(Parser)]
#[command(name = "ghgrab", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    url: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    Set {
        #[arg(long)]
        token: String,
    },

    Unset,

    List,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Config { action }) => match action {
            ConfigAction::Set { token } => {
                let mut config = Config::load()?;
                config.github_token = Some(token);
                config.save()?;
                println!("✅ GitHub token saved successfully!");
            }
            ConfigAction::Unset => {
                let mut config = Config::load()?;
                config.github_token = None;
                config.save()?;
                println!("✅ GitHub token removed successfully!");
            }
            ConfigAction::List => {
                let config = Config::load().unwrap_or_default();
                if let Some(token) = &config.github_token {
                    let masked = if token.len() > 8 {
                        format!("{}...{}", &token[..4], &token[token.len() - 4..])
                    } else {
                        "********".to_string()
                    };
                    println!("GitHub Token: {}", masked);
                } else {
                    println!("GitHub Token: Not set");
                }
            }
        },
        None => {
            let config = Config::load().unwrap_or_default();
            ui::run_tui(cli.url, config.github_token).await?;
        }
    }

    Ok(())
}
