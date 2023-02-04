use serde::{Deserialize, Serialize};

fn main() {
    let mut coin: String = String::new();
    println!("Ingrese el nombre de la criptomoneda: ");

    let _ = std::io::stdin()
        .read_line(&mut coin)
        .expect("Error al leer la entrada");

    let result_price = get_coin(&coin);
    match result_price {
        Ok(price) => println!("El precio de {} es {}", coin, price),
        Err(error) => println!("Error: {}", error),
    }
}

fn get_coin(coin: &str) -> Result<String, ureq::Error> {
    // Request to API and get the price of the coin

    let body = ureq::get(&format!(
        "https://api.coingecko.com/api/v3/coins/{}?localization=false",
        coin
    ))
    .call()?
    .into_string()?;
    let coin_data: CoinData = serde_json::from_str(&body).unwrap();
    Ok(coin_data.market_data.current_price.usd.to_string())
}

#[derive(Serialize, Deserialize, Debug)]
struct CoinData {
    id: String,
    symbol: String,
    name: String,
    market_data: MarketData,
}

#[derive(Serialize, Deserialize, Debug)]
struct MarketData {
    current_price: Prices,
}

#[derive(Serialize, Deserialize, Debug)]
struct Prices {
    usd: f32,
}
