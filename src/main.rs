use log::LevelFilter;
use clap::{Parser, Subcommand};

mod config;
mod client;

#[macro_use] extern crate log;

#[derive(Parser)]
#[command(name = "bitstamp")]
#[command(about = "A console Bitstamp API client")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get ticker information for a market symbol
    Ticker {
        /// Market symbol (e.g., btcusd, ethusd)
        market_symbol: String,
    },
    /// List all available currencies
    Currencies {
        #[arg(short = 'b', long = "brief", help = "Show only currency codes")]
        brief: bool,
    },
    /// List all available markets
    Markets {
        #[arg(short = 'b', long = "brief", help = "Show only market symbols")]
        brief: bool,
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)  // Default to Info level
        .init();

    let cli = Cli::parse();
    let config = config::from_json("config.json");
    let bitstamp = client::Client::new(&config);

    match cli.command {
        Commands::Ticker { market_symbol } => {
            let ticker = bitstamp.get_ticker(market_symbol)?;
            info!("{:#?}", ticker);
        }
        Commands::Currencies { brief } => {
            let currencies = bitstamp.get_currencies()?;
            if brief {
                let currency_symbols: Vec<String> = currencies.iter().map(|c| c.currency.clone()).collect();
                info!("{:#?}", currency_symbols);
            } else {
                info!("{:#?}", currencies);
            }
        }
        Commands::Markets { brief } => {
            let markets = bitstamp.get_markets()?;
            if brief {
                let market_symbols: Vec<String> = markets.iter().map(|m| m.market_symbol.clone()).collect();
                info!("{:#?}", market_symbols);
            } else {
                info!("{:#?}", markets);
            }
        }
    }

    Ok(())
}
