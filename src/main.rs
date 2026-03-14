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

    #[arg(long, help = "Download files to current directory")]
    cwd: bool,

    #[arg(long, help = "Download files directly into target without repo folder")]
    no_folder: bool,

    #[arg(long, help = "One-time GitHub token (not stored)")]
    token: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Config {
        #[command(subcommand)]
        action: ConfigCommand,
    },
}

#[derive(Subcommand)]
enum ConfigCommand {
    Set {
        #[command(subcommand)]
        target: SetTarget,
    },

    Unset {
        #[command(subcommand)]
        target: UnsetTarget,
    },

    List,
}

#[derive(Subcommand)]
enum SetTarget {
    Token { value: String },

    Path { value: String },
}

#[derive(Subcommand)]
enum UnsetTarget {
    Token,

    Path,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Config { action }) => match action {
            ConfigCommand::Set { target } => match target {
                SetTarget::Token { value } => {
                    let mut config = Config::load()?;
                    config.github_token = Some(value);
                    config.save()?;
                    println!("✅ GitHub token saved successfully!");
                }
                SetTarget::Path { value } => {
                    if let Err(e) = Config::validate_path(&value) {
                        eprintln!("❌ Invalid path: {}", e);
                    } else {
                        let mut config = Config::load()?;
                        config.download_path = Some(value);
                        config.save()?;
                        println!("✅ Download path saved successfully!");
                    }
                }
            },
            ConfigCommand::Unset { target } => match target {
                UnsetTarget::Token => {
                    let mut config = Config::load()?;
                    config.github_token = None;
                    config.save()?;
                    println!("✅ GitHub token removed successfully!");
                }
                UnsetTarget::Path => {
                    let mut config = Config::load()?;
                    config.download_path = None;
                    config.save()?;
                    println!("✅ Download path removed successfully!");
                }
            },
            ConfigCommand::List => {
                let config = Config::load().unwrap_or_default();
                if let Some(token) = &config.github_token {
                    let masked = if token.len() > 8 {
                        format!("{}...{}", &token[..4], &token[token.len() - 4..])
                    } else {
                        "********".to_string()
                    };
                    println!("GitHub Token:  {}", masked);
                } else {
                    println!("GitHub Token:  Not set");
                }

                if let Some(path) = &config.download_path {
                    println!("Download Path: {}", path);
                } else {
                    println!("Download Path: Not set (using default Downloads folder)");
                }
            }
        },
        None => {
            let config = Config::load().unwrap_or_default();

            let url = cli.url;

            let download_path = config.download_path;

            let token = cli.token.or(config.github_token);
            ui::run_tui(url, token, download_path, cli.cwd, cli.no_folder).await?;
        }
    }

    Ok(())
}
