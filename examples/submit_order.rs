use btcturk_websockets::{ApiKeys, Client, Decimal, Error, SubmitOrderRequest, OrderMethod, OrderType};
use btcturk_websockets::rust_decimal::RoundingStrategy;
use std::env;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file

    let public_key = env::var("BTCTURK_PUBLIC_KEY").expect("BTCTURK_PUBLIC_KEY not set");
    let private_key = env::var("BTCTURK_PRIVATE_KEY").expect("BTCTURK_PRIVATE_KEY not set");

    let keys = ApiKeys::new(public_key, private_key);
    let client = Client::new("wss://ws-feed-sandbox.btcturk.com/", keys);
    
    // Get account balance
    let balances = client.get_account_balance().await?;
    let try_balance = balances
        .data
        .iter()
        .find(|b| b.asset == "TRY")
        .map(|b| b.free)
        .unwrap_or(Decimal::ZERO);

    println!("💰 TRY balance: {}", try_balance);

    let pair_symbol = "USDTTRY";
    
    // Get current market price and exchange info for price limits
    println!("📈 Fetching market data for {}...", pair_symbol);
    let ticker = client.get_ticker(Some(pair_symbol)).await?;
    let current_price = ticker.data.first()
        .map(|t| t.last)
        .ok_or("No ticker data found")?;
    println!("   Current price: {}", current_price);

    // Get exchange info for price limits
    let exchange_info = client.get_exchange_info().await?;
    let symbol_info = client.get_symbol_info(&exchange_info, pair_symbol)
        .ok_or_else(|| format!("Symbol {} not found in exchange info", pair_symbol))?;

    let min_price = symbol_info.minimum_limit_order_price
        .ok_or("No minimum price limit available")?;
    let max_price = symbol_info.maximum_limit_order_price
        .ok_or("No maximum price limit available")?;

    println!("   Price limits - Min: {}, Max: {}", min_price, max_price);
    println!("   Current price is {} ({:.2}% of max)", current_price, (current_price / max_price) * Decimal::ONE_HUNDRED);

    // Use current market price (or ask price for buy orders) as the order price
    // For buy orders, use ask price to get immediate execution
    let order_price = if let Some(t) = ticker.data.first() {
        let ask_price = t.ask;
        // Ensure price is within valid limits
        if ask_price < min_price {
            println!("⚠️  Ask price {} is below minimum {}, using minimum", ask_price, min_price);
            min_price
        } else if ask_price > max_price {
            println!("⚠️  Ask price {} is above maximum {}, using maximum", ask_price, max_price);
            max_price
        } else {
            ask_price
        }
    } else {
        current_price
    };

    println!("📝 Using order price: {}", order_price);

    let quantity_try = try_balance.min(Decimal::ONE_HUNDRED);
    let quantity_usdt = quantity_try / order_price;

    let quantity_final = quantity_usdt.round_dp_with_strategy(
        symbol_info.numerator_scale as u32,
        RoundingStrategy::MidpointAwayFromZero,
    );
    
    println!("🔄 Placing buy order:");
    println!("   Quantity: {} {} (scale: {} decimals)", quantity_final, symbol_info.name_normalized.split('_').next().unwrap_or("USDT"), symbol_info.numerator_scale);
    println!("   Price: {} TRY", order_price);
    println!("   Total cost: {} TRY", quantity_try);

    let order_request = SubmitOrderRequest {
        quantity: quantity_final,
        price: Some(order_price),
        stop_price: None,
        order_method: OrderMethod::Limit,
        order_type: OrderType::Buy,
        pair_symbol: pair_symbol.to_string(),
        new_order_client_id: None,
    };

    match client.submit_order(order_request).await {
        Ok(response) => {
            println!("✅ Order submitted successfully: {:?}", response);
        }
        Err(Error::Api { status, body }) => {
            eprintln!("❌ BtcTurk rejected the order ({}): {}", status, body);
            eprintln!("   Make sure the price ({}) is between {} and {}", order_price, min_price, max_price);
            return Err(format!("order rejected with status {}", status).into());
        }
        Err(e) => {
            eprintln!("❌ Failed to submit order: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
