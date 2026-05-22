use clap::Parser;

#[derive(Parser)]
#[command(name = "bridge")]
#[command(about = "Bridge between opencode and Neovim.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    /// Process opencode hooks via stdin/stdout JSON protocol
    #[command(name = "hook")]
    Hook,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hook => bridge::handler::handle_hook(),
    }
}
