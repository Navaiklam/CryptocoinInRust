use serde_json::Value;


fn main() {
    println!("Ingrese la abreviatura de su moneda digital favoritas, como BTC, ETH, LTC");
    let mut coin: String = String::new();
    let _ = std::io::stdin()
                .read_line(&mut coin)
                .expect("ocurrio un error anda a saber qué onda");

    println!("Ingrese el Symbol de la moneda");
    let mut symbol: String = String::new();
    let _ = std::io::stdin()
                .read_line(&mut symbol)
                .expect("ocurrio un error anda a saber qué onda");

    let r_precio: Result<String, ureq::Error> = get_precio(&coin, &symbol);
    
    match r_precio {
        Ok(precio) => println!("El precio actual es: {} en {}", &precio, &symbol),
        Err(error) => println!("Error: {}", error),
       
        }
   
}
fn get_precio(coin: &str, symbol: &str) -> Result<String, ureq::Error> {
    let sy = symbol.trim();
    dbg!(sy);
    let body: String = ureq::get(&format!("https://min-api.cryptocompare.com/data/price?fsym={}&tsyms={}", &coin, &sy))
        .call()?
        .into_string()?;
    let parsed: Value = serde_json::from_str(&body).unwrap();
   
    println!("el json {}", &body);
    
    println!("El tipo de moneda {}", &symbol);
   
    Ok(parsed[&sy].to_string())
}