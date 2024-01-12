// Comienza la función principal del programa
fn main() {
    // Solicita al usuario que ingrese la abreviatura de su moneda digital favorita
    println!("Ingrese la abreviatura de su moneda digital favorita, como BTC, ETH, LTC");

    // Declara una variable mutable 'coin' para almacenar la abreviatura de la moneda
    let mut coin: String = String::new();

    // Lee la entrada del usuario y la asigna a la variable 'coin'
    let _ = std::io::stdin()
        .read_line(&mut coin)
        .expect("Ocurrió un error, anda a saber qué onda");

    // Llama a la función 'get_precio' para obtener el precio de la moneda ingresada
    let r_precio: Result<String, ureq::Error> = get_precio(&coin);

    // Evalúa el resultado de la llamada a 'get_precio' y muestra el precio o el error correspondiente
    match r_precio {
        Ok(precio) => println!("El precio actual es: {}", precio),
        Err(error) => println!("Error: {}", error),
    }
}

// Función que obtiene el precio de una moneda digital a partir de su abreviatura
fn get_precio(coin: &str) -> Result<String, ureq::Error> {
    // Realiza una solicitud GET a la API de CryptoCompare para obtener el precio en euros de la moneda
    let body: String = ureq::get(&format!("https://min-api.cryptocompare.com/data/price?fsym={}&tsyms=EUR", coin))
        .call()? // Realiza la llamada a la API y maneja posibles errores
        .into_string()?; // Convierte la respuesta en una cadena de texto

    // Retorna el cuerpo de la respuesta como un Resultado exitoso
    Ok(body)
}

1. **`fn main() {`**: 
   - Define la función principal del programa.

2. **`println!("Ingrese la abreviatura de su moneda digital favorita, como BTC, ETH, LTC");`**: 
   - Imprime un mensaje solicitando al usuario que ingrese la abreviatura de su moneda digital favorita.

3. **`let mut coin: String = String::new();`**: 
   - Declara una variable mutable llamada 'coin' para almacenar la abreviatura de la moneda.

4. **`let _ = std::io::stdin().read_line(&mut coin).expect("Ocurrió un error, anda a saber qué onda");`**: 
   - Lee la entrada del usuario y la asigna a la variable 'coin'. El `_` se usa para indicar que no nos importa el resultado de esta operación, pero queremos manejar cualquier error que pueda ocurrir.

5. **`let r_precio: Result<String, ureq::Error> = get_precio(&coin);`**: 
   - Llama a la función `get_precio` para obtener el precio de la moneda ingresada y almacena el resultado en un `Result`, que puede ser un valor exitoso (`Ok`) o un error (`Err`).

6. **`match r_precio { Ok(precio) => println!("El precio actual es: {}", precio), Err(error) => println!("Error: {}", error), }`**: 
   - Evalúa el resultado de la llamada a `get_precio` y muestra el precio si es exitoso, o muestra un mensaje de error en caso de que ocurra algún problema.

7. **`fn get_precio(coin: &str) -> Result<String, ureq::Error> {`**: 
   - Define la función `get_precio` que toma la abreviatura de la moneda y retorna un `Result` que contiene una cadena de texto o un error de la librería `ureq`.

8. **`let body: String = ureq::get(&format!("https://min-api.cryptocompare.com/data/price?fsym={}&tsyms=EUR", coin)).call()? .into_string()?;`**: 
   - Realiza una solicitud GET a la API de CryptoCompare para obtener el precio en euros de la moneda. `ureq::get` crea una solicitud, `call` la ejecuta y `into_string` convierte la respuesta en una cadena de texto. Los `?` manejan posibles errores y propagan el error hacia arriba en caso de que ocurra.

9. **`Ok(body)`**: 
   - Retorna el cuerpo de la respuesta como un `Result` exitoso que contiene una cadena de texto.
