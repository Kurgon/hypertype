use clap::Parser;
use colored::*;
mod init;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    println!("{}", "⚡HYPERTYPE ⚡".bold().yellow());
    println!("{} {}", "Author:".green(), "Kurgon".purple());
    println!("{} {}\n", "Version".green(), "0.1.0".purple());

    init::init().await?;

    Ok(())
}
