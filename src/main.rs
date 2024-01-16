use serde_json::{Value, Error};

fn r_menu() -> Result<i32, Error>{
    Ok(1)
}


fn main() {
    loop {
        match r_menu() {
            Ok(_menu) => {
                // Imprime las opciones del menú
                println!("1 - Precio crypto");
                println!("2 - Exit");
    
                // Lee la opción del usuario
                let mut menu_input = String::new();
                std::io::stdin().read_line(&mut menu_input).expect("Error al leer la opción");
    
                // Parsea la opción a un número entero
                let menu: u32 = match menu_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Sólo debes de ingresar un número");
                        continue;
                    }
                };
    
                // Realiza acciones según la opción seleccionada
                match menu {
                    1 => {
                        // Opción para obtener el precio de una criptomoneda
                        option_lastprice();
                    }
                    2 => {
                        // Opción para salir del programa
                        println!("Salir");
                        break;
                    }
                    _ => {
                        // Opción no válida
                        println!("Opción no válida. Por favor, elija una opción válida.");
                        continue;
                    }
                }
            }
            Err(error) => println!("Error: {}", error),
        }
    }
    
}


fn option_lastprice(){
    println!("Ingrese la abreviatura de su moneda digital favoritas, como BTC, ETH, LTC");
    let mut coin: String = String::new();
    let _ = std::io::stdin()
        .read_line(&mut coin)
        .expect("Ocurrió un error, anda a saber qué onda");

    println!("Ingrese el Symbol de la moneda");
    let mut symbol: String = String::new();
    let _ = std::io::stdin()
        .read_line(&mut symbol)
        .expect("Ocurrió un error, anda a saber qué onda");

    let r_precio: Result<String, ureq::Error> = get_precio(&coin, &symbol);

    match r_precio {
        Ok(precio) => println!("El precio actual es: {} en {}", &precio, &symbol),
        Err(error) => println!("Error: {}", error),
    }
    main();

}


fn get_precio(coin: &str, symbol: &str) -> Result<String, ureq::Error> {
    let sy = symbol.trim();
    dbg!(sy);
    dbg!(symbol);
    let body: String = ureq::get(&format!("https://min-api.cryptocompare.com/data/price?fsym={}&tsyms={}", &coin, &sy))
        .call()?
        .into_string()?;
    let parsed: Value = serde_json::from_str(&body).unwrap();
   
    println!("el json {}", &body);
    
    println!("El tipo de moneda {}", &symbol);
   
    Ok(parsed[&sy].to_string())
}
