fn main() {
    let mut x = 5;
    let y = 7;
    println!("Hola choquito");
    let suma = x + y;
    println!("El valor de x es: {}, y el valor de y es: {}", x, y);
    println!("La suma de x + y es: {}", suma);

    x = 9;
    println!("El valor de x ahora es: {}", x);

    let edad = 20;
    if edad >= 18 {
        println!("Ya podés votar.. sos grandesito");
    } else {
    println!("No podés votar, sos pelao");
    }

    //LOOP
    //Hacer uso del bucle Loop, para verificar una "contraseña". intentos = 0; intentos += 1; imprimir la cantidad
    //de intentos que lleva.. verificar si llegó a los 3 intentos. Si llegó, que se salga y que muestre:
    //Cuenta bloqueada. Si no llegó, que siga sumando los intentos.

    let mut intentos = 0;
    loop {
        intentos += 1;
        println!("Ya llevas {} intentos", intentos);

        if intentos == 3 {
            println!("Cuenta bloqueada, demasiados intentos");
            break;
        }
    }

    //WHILE
    //Simulador de carga de batería:
    // bateria = 100; usar el bucle while, mientras que la batería sea mayor que cero "0", devuelva un mensaje
    //de la cantidad de batería que aún queda. En cada iteración, va a reducir 20 la batería.
    //finalizado que muestre el mensaje: Te quedaste sin bateria.


    let mut bateria = 100;
    while bateria > 0 {
        println!("Aún te queda {}% de batería", bateria);
        bateria -= 20;
    }
    println!("Te quedaste sin bateria");

    //FOR
    //Calculadora de la tabla del 5: Usar un "for" que recorra todos los números del 1 al 10, y
    //que devuelva cada números multiplicado por 5.

    for i in 1..=10 {
        println!("5 x {} = {}",i, i*5);
    }

    //FUNCIONES
    //Hacer una función que convierta los grados centígrados a grados Fahrenheit.
    //Formula = (gc * 1.8) + 32

    let temp: f32 = 20.5;    
    println!("La temperatura de {} grados centígrados, es igual a {} grados fahrenheit.", temp, caf(temp));
}

fn caf(c: f32) ->f32 {
    (c * 1.8) + 32.0
}
