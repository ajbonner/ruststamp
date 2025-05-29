use crate::client::ApiConfig;
use clap::{Parser, Subcommand};
use clap::builder::styling;
use log::{LevelFilter, info};

mod client;
mod error;

const CLI_STYLE: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Blue.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());


#[derive(Parser)]
#[command(name = "ruststamp")]
#[command(about = "A console Bitstamp API client written in Rust")]
#[command(version)]
#[command(color = clap::ColorChoice::Auto)]
#[command(styles = CLI_STYLE)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get ticker information for a market symbol
    Ticker {
        /// Market symbol (e.g., btcusd, xrpgbb). Use `markets` command to list all available markets
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
    },
    /// Get order book for a market symbol
    OrderBook {
        /// Market symbol (e.g., btcusd, xrpgbb). Use `markets` command to list all available markets
        market_symbol: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info) // Default to Info level
        .init();

    let cli = Cli::parse();
    let config = ApiConfig::from_json("config.json");
    let bitstamp = client::RestClient::new(&config);

    match cli.command {
        Commands::Ticker { market_symbol } => {
            let ticker = bitstamp.get_ticker(market_symbol.as_str())?;
            info!("{:#?}", ticker);
        }
        Commands::Currencies { brief } => {
            let currencies = bitstamp.get_currencies()?;
            if brief {
                let currency_symbols: Vec<String> =
                    currencies.iter().map(|c| c.currency.clone()).collect();
                info!("{:#?}", currency_symbols);
            } else {
                info!("{:#?}", currencies);
            }
        }
        Commands::Markets { brief } => {
            let markets = bitstamp.get_markets()?;
            if brief {
                let market_symbols: Vec<String> =
                    markets.iter().map(|m| m.market_symbol.clone()).collect();
                info!("{:#?}", market_symbols);
            } else {
                info!("{:#?}", markets);
            }
        }
        Commands::OrderBook { market_symbol } => {
            let order_book = bitstamp.get_order_book(market_symbol.as_str())?;
            info!("{:#?}", order_book);
        }
    }

    Ok(())
}
