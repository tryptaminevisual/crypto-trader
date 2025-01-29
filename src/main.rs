use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use csv::{Writer, WriterBuilder, ReaderBuilder};
use dotenv::dotenv;
use chrono::Local;
use textplots::{Chart, Plot, Shape};

#[derive(Debug, Serialize, Deserialize)]
struct Coin {
    symbol: String,
    investment: f64,
    purchase_price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    api_key: String,
    coins: Vec<Coin>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();

    // Read configuration file
    let config = read_config("config.json").await?;

    // Initialize CSV writer
    let file = File::create("data.csv")?;
    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_writer(file);

    // Write header
    writer.write_record(&[
        "Timestamp",
        "Coin",
        "Current Price",
        "Investment",
        "Current Value",
        "ROI (%)",
    ])?;

    // Build the query string for specific coins
    let symbols: Vec<String> = config.coins.iter().map(|coin| coin.symbol.clone()).collect();
    let symbol_query = symbols.join(",");

    // Fetch CoinMarketCap data
    let client = reqwest::Client::new();
    let response = client
        .get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest")
        .header("Accepts", "application/json")
        .header("X-CMC_PRO_API_KEY", config.api_key.as_str())
        .query(&[("symbol", &symbol_query)]) // Add the symbol query parameter
        .send()
        .await?;

    let json: Value = response.json().await?;

    // Process data and calculate statistics
    let (total_investment, total_current_value, investments, current_values, rois, coins) =
        process_data(json, &config.coins, &mut writer).await?;

    // Calculate overall ROI
    let overall_roi = ((total_current_value - total_investment) / total_investment) * 100.0;

    // Print detailed statistics
    println!("--- Portfolio Summary ---");
    println!("Total Investment: ${:.2}", total_investment);
    println!("Total Current Value: ${:.2}", total_current_value);
    println!("Overall ROI: {:.2}%", overall_roi);

    // Generate ASCII charts
    generate_ascii_charts(&coins, &investments, &current_values, &rois);

    Ok(())
}

async fn read_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let config: Config = serde_json::from_reader(file)?;
    Ok(config)
}

async fn process_data(
    json: Value,
    coins: &Vec<Coin>,
    writer: &mut Writer<File>,
) -> Result<(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>, Vec<String>), Box<dyn std::error::Error>> {
    let timestamp = Local::now().to_rfc3339();

    let mut total_investment = 0.0;
    let mut total_current_value = 0.0;

    let mut investments = Vec::new();
    let mut current_values = Vec::new();
    let mut rois = Vec::new();
    let mut coin_names = Vec::new();

    for coin in coins {
        let symbol = &coin.symbol;
        let investment = coin.investment;
        let purchase_price = coin.purchase_price;

        // Extract current price from JSON
        let current_price = json
            .get("data")
            .and_then(|data| data.get(symbol))
            .and_then(|data| data.get("quote"))
            .and_then(|quote| quote.get("USD"))
            .and_then(|price| price.get("price"))
            .and_then(|p| p.as_f64())
            .unwrap_or(0.0);

        // Calculate the number of coins purchased
        let number_of_coins = investment / purchase_price;

        // Calculate current value
        let current_value = number_of_coins * current_price;

        // Calculate return on investment
        let roi = ((current_value - investment) / investment) * 100.0;

        // Update totals
        total_investment += investment;
        total_current_value += current_value;

        // Store data for ASCII charts
        investments.push(investment);
        current_values.push(current_value);
        rois.push(roi);
        coin_names.push(symbol.clone());

        // Write to CSV
        writer.write_record(&[
            &timestamp,
            symbol,
            &current_price.to_string(),
            &investment.to_string(),
            &current_value.to_string(),
            &roi.to_string(),
        ])?;
    }

    Ok((
        total_investment,
        total_current_value,
        investments,
        current_values,
        rois,
        coin_names,
    ))
}

fn generate_ascii_charts(
    coins: &Vec<String>,
    investments: &Vec<f64>,
    current_values: &Vec<f64>,
    rois: &Vec<f64>,
) {
    // ASCII Chart: Investment vs Current Value
    println!("\n--- Investment vs Current Value ---");
    let investment_vs_value: Vec<(f32, f32)> = investments
        .iter()
        .zip(current_values.iter())
        .enumerate()
        .map(|(i, (&investment, &current_value))| (i as f32, investment as f32 - current_value as f32))
        .collect();

    Chart::new(80, 20, 0.0, investment_vs_value.len() as f32)
        .lineplot(&Shape::Bars(&investment_vs_value))
        .display();

    // ASCII Chart: ROI
    println!("\n--- Return on Investment (ROI) ---");
    let roi_chart: Vec<(f32, f32)> = rois
        .iter()
        .enumerate()
        .map(|(i, &roi)| (i as f32, roi as f32))
        .collect();

    Chart::new(80, 20, 0.0, roi_chart.len() as f32)
        .lineplot(&Shape::Lines(&roi_chart))
        .display();
}
