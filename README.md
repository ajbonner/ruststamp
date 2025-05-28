# Bitstamp API Client

A command-line interface (CLI) client for interacting with the Bitstamp cryptocurrency exchange API. This Rust-based tool provides easy access to market data and trading information.

## Features

- Get ticker information for any market pair
- List all available currencies
- List all available markets
- Simple and intuitive command-line interface
- JSON configuration support

## Installation

1. Ensure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/)
2. Clone this repository
3. Build the project:
```bash
cargo build --release
```

## Configuration

A skeleton configuration file `config.json.dist` is provided in the project root. To set up your configuration:

1. Copy `config.json.dist` to `config.json`:
```bash
cp config.json.dist config.json
```

2. Note: Currently, this client only uses Bitstamp's public API endpoints, so no API credentials are required. The configuration entries for client credentials are included for future expansion to the authenticated endpoints.

## Usage

The client provides several commands:

### Get Ticker Information

```bash
./bitstamp ticker <market_symbol>
```

Example:
```bash
./bitstamp ticker btcusd
```

### List Currencies

```bash
./bitstamp currencies
```

For brief output (currency codes only):
```bash
./bitstamp currencies --brief (or -b)
```

### List Markets

```bash
./bitstamp markets
```

For brief output (market symbols only):
```bash
./bitstamp markets --brief (or -b)
```

## Dependencies

- clap: Command-line argument parsing
- log: Logging functionality
- pretty_env_logger: Formatted logging output
- serde: Serialization/deserialization framework
- serde_json: JSON serialization/deserialization support

## License

This project is licensed under the terms of the license included in the repository.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 