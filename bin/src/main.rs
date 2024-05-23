use clap::Parser;
use cmd::{fetch::fetch_and_gen, submit::submit_and_check};

mod action;
mod cmd;
mod graphql;
mod model;
mod util;

#[tokio::main]
async fn main() {
    let cli = cmd::Cli::parse();

    match cli.command {
        cmd::Commands::Fetch(args) => fetch_and_gen(args).await,
        cmd::Commands::Submit(args) => submit_and_check(args).await,
    }
}
