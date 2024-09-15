use std::io;

fn main() {

    // Solicitar al usuario que ingrese un número
    println!("Por favor, ingresa un número (tipo entero y positivo):");

    // Crear una variable mutable para almacenar la entrada
    let mut input = String::new();

    // Leer la entrada del usuario
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la línea");

    // Convertir la entrada en un número entero (u32) y manejar errores
    let number: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, ingresa un número entero válido.");
            return;
        }
    };

    let number_fibonacci = n_fibonacci_number(number);

    println!("Fibonacci: {number_fibonacci}")
}

fn n_fibonacci_number(n: u32) -> u32 {
    let mut n1 = 0;
    let mut n2 = 1;
    let mut fibonacci = 0;

    for _i in 1..n {
        fibonacci = n1 + n2;

        n1 = n2;
        n2 = fibonacci;
    }

    fibonacci
}

