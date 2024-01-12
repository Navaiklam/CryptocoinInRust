

fn main() {
    println!("Ingrese la abreviatura de su moneda digital favoritas, como BTC, ETH, LTC");
    let mut coin: String = String::new();
    let _ = std::io::stdin()
        .read_line(&mut coin)
        .expect("ocurrio un error anda a saber qué onda");
    
    let r_precio: Result<String, ureq::Error> = get_precio(&coin);
    match r_precio {
        Ok(precio) => println!("El precio actual es: {}", precio),
        Err(error) => println!("Error: {}", error),
    }
}
fn get_precio(coin: &str) -> Result<String, ureq::Error> {
  
    let body: String = ureq::get(&format!("https://min-api.cryptocompare.com/data/price?fsym={}&tsyms=EUR", coin))
        .call()?
        .into_string()?;
    Ok(body)
}

