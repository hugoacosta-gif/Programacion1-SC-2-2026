fn main() {
    println!("Hola mundo!");
    print!("------------------------------\n");
    //cuenta regresiva hasta 1
    let mut cuenta = 10;
    while cuenta > 0 {
        println!("cuenta en.. {}", cuenta);
        cuenta -= 1;
    }
    print!("------------------------------\n");
    //sumar todos los numeros del 1 al 10
    let mut suma = 0;
    for i in 1..=10 {
        suma = suma + i;
    }
    println!("La suma del 1 al 10 es: {}", suma);
    print!("------------------------------\n");
    for m in 1..=30{
        match (m%3,m%5) { //reconoce patrones
            (0,0) => println!("fizzbuzz"),
            (0,_) => println!("fizz"),
            (_,0) => println!("buzz"),            
            _ => println!("{}", m),
        }
    }    
    print!("------------------------------\n");    
    println!("el cuadrado es: {}", cuadrado(5));
    println!("El numero 17 es par?: {}", espar(17));
    println!("La temperatura de 20 GC en farenheit es: {}", convertir(20.0));
    println!("el dia 3 es laboral?: {}", laboral(3));
    println!("el fibonacci de 8 es: {}", fibonacci(8));
    println!("El mes 2 tiene {} dias", dias_mes(2));
    println!("El 3 corresponde al dia: {}", diasemana(3));
}

fn cuadrado(n:i32) ->i32 {
    n*n
}

fn convertir(c: f64) ->f64 {
    //esta funcion debe convertir grados centigrados a grados farenheit.
    c*33.8
}

fn espar(p: i64) ->bool {
    //funcion que devuelve si un numero es par.
    if p % 2  == 0 { return true} else {return false}
}

fn diasemana(dia: u32) ->&'static str {
    match dia {
        1 => "lunes",
        2 => "martes",
        3 => "miercoles",
        4 => "jueves",
        5 => "viernes",
        6 => "sabado",
        7 => "domingo",
        _ => "no se conoce",
    }
}

//1.- funcion que dado un dia de la semana, devuelva si es laboral o no (lunes - viernes)
fn laboral(dia: u32) -> bool {
    match dia {
        1 | 2 | 3 | 4 | 5 => true,
        6 | 7 => false,
        _ => false,
    }
}
//2.- funcion para que resuelva Fibonacci.
fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut a = 0;
        let mut b = 1;
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        b
    }
}

//3- usando match realizar una funcion que dado el mes, devuelva la cantidad de dias que tiene ese mes. Ejemplo: febrero (2) - 28
fn dias_mes(mes: u32) -> u32 {
    match mes {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => 28, // Ignorando años bisiestos
        _ => 0, // Mes no válido
    }
}