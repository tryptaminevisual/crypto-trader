# Crypto Trader

Crypto Trader is a Rust-based command-line application that helps you track your cryptocurrency investments. It fetches real-time cryptocurrency prices using the CoinMarketCap API, calculates your portfolio's performance, and provides detailed statistics. Additionally, it generates ASCII charts for a visual representation of your investments and ROI.

---

## Features

- Fetches real-time cryptocurrency prices from the CoinMarketCap API.
- Tracks your investments and calculates:
  - Total investment
  - Current portfolio value
  - Return on Investment (ROI) for each coin and overall portfolio.
- Exports data to a CSV file for easy record-keeping.
- Generates ASCII charts for:
  - Investment vs Current Value
  - ROI (Return on Investment)
- Fully configurable via a `config.json` file.


## Installation

### Prerequisites

- Rust (latest stable version)
- A CoinMarketCap API key
- A `config.json` file with your investment details

### Clone the Repository

```bash
git clone https://github.com/tryptaminevisual/crypto-trader.git
cd crypto_trader
```
## Install Dependencies
Ensure you have the required dependencies installed. Add them to your Cargo.toml if necessary:
``` toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
dotenv = "0.15"
chrono = { version = "0.4", features = ["serde"] }
csv = { version = "1.2", default-features = false }
textplots = "0.8"
```
## Configuration 
Create a config.json file in the root directory with the following structure:
```json
{
  "api_key": "your_coinmarketcap_api_key",
  "coins": [
    {
      "symbol": "BTC",
      "investment": 1000.0,
      "purchase_price": 50000.0
    },
    {
      "symbol": "ETH",
      "investment": 500.0,
      "purchase_price": 2500.0
    }
  ]
}
```
- api_key: Your CoinMarketCap API key.
- coins: A list of coins you want to track, including:
- symbol: The cryptocurrency symbol (e.g., BTC, ETH).
- investment: The amount you invested in USD.
- purchase_price: The price at which you purchased the coin.

## Usage
Run the Application
  - Build and run the application:
    ```bash
    cargo run
    ```
The program will:
  - Fetch real-time cryptocurrency prices.
  - Calculate your portfolio's performance.
  - Save the data to a data.csv file.
  - Display ASCII charts for your investments and ROI.
## Output
CSV File
The program generates a data.csv file with the following columns:

  - Timestamp: The time the data was fetched.
  - Coin: The cryptocurrency symbol.
  - Current Price: The current price of the coin.
  - Investment: The amount you invested.
  - Current Value: The current value of your investment.
  - ROI (%): The return on investment percentage.
ASCII Charts
The program displays two ASCII charts in the terminal:
  Investment vs Current Value:
    - A bar chart comparing your initial investment to the current value of each coin.
  Return on Investment (ROI):
    - A line chart showing the ROI percentage for each coin.

## Code Review 
Main Function
The main function orchestrates the program's workflow:

  - Loads environment variables.
  - Reads the config.json file.
  - Fetches cryptocurrency data from the CoinMarketCap API.
  - Processes the data to calculate statistics.
  - Writes the data to a CSV file.
  - Generates ASCII charts for visualization.
  - Key Functions
  - read_config
  - Reads the config.json file and deserializes it into a Config struct.

process_data
Processes the API response to calculate:

  - Current value of each coin.
  - ROI for each coin.
  - Total investment and current portfolio value.
  - Writes the processed data to the CSV file.

generate_ascii_charts
Generates ASCII charts for:

  - Investment vs Current Value
  - ROI
  - 
## Dependencies
The project uses the following Rust crates:

  - reqwest: For making HTTP requests to the CoinMarketCap API.
  - serde: For serializing and deserializing JSON data.
  - serde_json: For working with JSON data.
  - tokio: For asynchronous programming.
  - dotenv: For loading environment variables.
  - chrono: For working with timestamps.
  - csv: For writing data to CSV files.
  - textplots: For generating ASCII charts.

## Example
Sample Output
```bash
--- Portfolio Summary ---
Total Investment: $500.00
Total Current Value: $491.80
Overall ROI: -1.64%

--- Investment vs Current Value ---
⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ 20.3
⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡗⠲⠒⠖⠲⠒⠖⠲⡆⠄⠠⠀⠄⠠⠀⠄⣠⣀⣄⣠⣀⣄⣠⣀⡖⠲⠒⠖⠲⠒⠖⠲⡆⠄⠠⠀⠄⠠⠀⠄⠀
⡇⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀
⠁⠀⠀⠀⠀⠀⠀⠀⠉⠉⠉⠉⠉⠉⠉⠉⠁⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀ -9.8
0.0                                  5.0


--- Return on Investment (ROI) ---
⡁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠤⠒⠉⠉⠒⠢⠤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ 13.1
⠄⠀⠀⠀⠀⠀⠀⠀⠀⣀⠤⠒⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠒⠒⠒⠤⠤⢄⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠃⠈⠀⠁⠈⠀⣁⠜⠉⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠁⠁⠈⠀⠁⠈⠀⠁⠀
⡁⠀⠀⢀⠤⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠄⡠⠒⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ -20.3
0.0                                  5.0
```

## Contributing
Contributions are welcome! Feel free to open an issue or submit a pull request.

## License
This project is licensed under the MIT License. See the LICENSE file for details.
