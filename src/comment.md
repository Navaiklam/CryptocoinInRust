// **Script en Rust con Explicaciones**

```rust
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

    // Solicita al usuario que ingrese el símbolo de la moneda
    println!("Ingrese el símbolo de la moneda");

    // Declara una variable mutable 'symbol' para almacenar el símbolo de la moneda
    let mut symbol: String = String::new();

    // Lee la entrada del usuario y la asigna a la variable 'symbol'
    let _ = std::io::stdin()
        .read_line(&mut symbol)
        .expect("Ocurrió un error, anda a saber qué onda");

    // Llama a la función 'get_precio' para obtener el precio de la moneda ingresada
    let r_precio: Result<String, ureq::Error> = get_precio(&coin, &symbol);

    // Evalúa el resultado de la llamada a 'get_precio' y muestra el precio o el error correspondiente
    match r_precio {
        Ok(precio) => println!("El precio actual es: {} en {}", &precio, &symbol),
        Err(error) => println!("Error: {}", error),
    }
}

// Función que obtiene el precio de una moneda digital a partir de su abreviatura y símbolo
fn get_precio(coin: &str, symbol: &str) -> Result<String, ureq::Error> {
    // Elimina posibles espacios en blanco alrededor del símbolo de la moneda
    let sy = symbol.trim();
    // Imprime el valor del símbolo depurado utilizando la macro dbg!
    dbg!(sy);

    // Realiza una solicitud GET a la API de CryptoCompare para obtener el precio de la moneda
    let body: String = ureq::get(&format!("https://min-api.cryptocompare.com/data/price?fsym={}&tsyms={}", &coin, &sy))
        .call()? // Realiza la llamada a la API y maneja posibles errores
        .into_string()?; // Convierte la respuesta en una cadena de texto

    // Imprime el JSON de la respuesta
    println!("el json {}", &body);

    // Imprime el tipo de moneda original
    println!("El tipo de moneda {}", &symbol);

    // Convierte el cuerpo de la respuesta en un objeto JSON utilizando la librería serde_json
    let parsed: Value = serde_json::from_str(&body).unwrap();

    // Retorna el valor correspondiente al símbolo depurado como un Resultado exitoso
    Ok(parsed[&sy].to_string())
}



# Experiencia de Depuración en Rust

## Descripción del Problema

Durante el desarrollo de mi proyecto Rust, encontré un problema relacionado con la variable `symbol`. Resultó que la variable contenía un salto de línea (`\n`), lo que afectaba la construcción de la URL y causaba problemas en las solicitudes a la API. Sin embargo, el problema real se manifestaba al intentar acceder a propiedades en el objeto JSON (`parsed`) al final del proceso.

## Descubrimiento del Problema

Para comprender la naturaleza del problema, utilicé declaraciones `println!` para imprimir los valores de las variables relevantes durante la ejecución del programa. Fue a través de esta técnica de depuración que identifiqué que la variable `symbol` contenía un salto de línea.

# Solución y Resultados

Adicionalmente, utilicé la macro `dbg!(sy)` para imprimir el valor de `symbol` después de aplicar `trim`, lo que fue fundamental para confirmar que los espacios en blanco se eliminaron correctamente.
Después de realizar estos ajustes, el script comenzó a funcionar como se esperaba. La solución radicaba en la correcta manipulación de la variable `symbol` antes de usarla en la construcción de la URL y al acceder a propiedades en el objeto JSON (`parsed`).

## Lecciones Aprendidas

Esta experiencia me enseñó la importancia de realizar una depuración efectiva al enfrentar problemas en el código. Utilizar declaraciones de impresión y técnicas de depuración en Rust, como `dbg!`, resultaron ser herramientas valiosas para identificar y solucionar el problema, especialmente cuando la raíz del problema no estaba inicialmente clara.

## Cómo Usar la Depuración en Rust

- Utiliza declaraciones `println!` para imprimir valores de variables relevantes.
- Aplica métodos como `trim()` para manipular y limpiar cadenas de texto.
- Considera el uso de macros de depuración como `dbg!` para obtener información detallada durante la ejecución.
- Explora opciones avanzadas como el modo de depuración, la macro `assert!`, y depuradores interactivos (GDB).


## Proceso de Depuración

La solución implicó aplicar el método `trim()` a la variable `symbol` para eliminar cualquier espacio en blanco alrededor del símbolo:

```rust
let sy = symbol.trim();
