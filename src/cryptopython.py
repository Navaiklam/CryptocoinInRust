import requests
import json

def r_menu():
    return 1

def main():
    while True:
        menu = r_menu()
        print_menu_options()
    
        # Lee la opción del usuario
        menu_input = input("Ingrese la opción: ")
    
        # Parsea la opción a un número entero
        try:
            menu = int(menu_input)
        except ValueError:
            print("Sólo debes de ingresar un número")
            continue
    
        # Realiza acciones según la opción seleccionada
        actions = {
            1: option_lastprice,
            2: exit_program,
        }
        selected_action = actions.get(menu, invalid_option)
        selected_action()

def print_menu_options():
    print("1 - Precio crypto")
    print("2 - Exit")

def option_lastprice():
    print("Ingrese la abreviatura de su moneda digital favoritas, como BTC, ETH, LTC")
    coin = input()
    print("Ingrese el Symbol de la moneda")
    symbol = input()

    try:
        precio = get_precio(coin, symbol)
        print(f"El precio actual es: {precio} en {symbol}")
    except Exception as error:
        print(f"Error: {error}")

def get_precio(coin, symbol):
    sy = symbol.strip()
    print(sy)
    print(symbol)
    
    url = f"https://min-api.cryptocompare.com/data/price?fsym={coin}&tsyms={sy}"
    response = requests.get(url)
    response.raise_for_status()
    
    body = response.text
    parsed = json.loads(body)

    print(f"el json {body}")
    print(f"El tipo de moneda {symbol}")

    return str(parsed[sy])

def exit_program():
    print("Salir")
    exit(0)

def invalid_option():
    print("Opción no válida. Por favor, elija una opción válida.")

if __name__ == "__main__":
    main()
