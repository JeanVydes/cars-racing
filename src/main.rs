///
/// # Ejemplo de carrera de carros
///
/// Jean Vides
/// Universidad del Magdalena
/// Introduccion a Ingenieria de Sistemas
/// 2023-1
///
use rand::Rng; // Libreria para generar numeros aleatorios
use std::collections::HashMap; // Libreria para almacenar informacion de la carrera

fn main() {
    // Lista de carros disponibles
    let cars = [
        "Porsche 911",
        "Audi R8",
        "Ferrari 488 GTB",
        "Lamborghini Aventador",
        "McLaren 720S",
        "Mercedes-AMG GT",
        "BMW M4",
        "Chevrolet Corvette",
        "Ford Mustang",
        "Dodge Challenger",
    ];

    print!("{}[2J", 27 as char); // Limpiar pantalla

    let mut number_of_cars = 0;
    // Preguntando por el numero de carros que van a competir
    while number_of_cars <= 1 || number_of_cars > cars.len() as i32 {
        println!("Cuantos carros quieres que compitan? (1-{})", cars.len());
        let mut n_cars_string = String::new();

        std::io::stdin()
            .read_line(&mut n_cars_string)
            .ok()
            .expect("error");

        number_of_cars = match n_cars_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Solo numeros!");
                0
            }
        };
    }

    print!("{}[2J", 27 as char); // Limpiar pantalla

    // Mostrando todos los carros disponibles para elegir
    let mut cars_to_compete: Vec<i32> = vec![];
    for (position, car) in cars.iter().enumerate() {
        println!("{}. {}", position, car);
    }

    // Obteniendo los vehiculos que van a competir
    while cars_to_compete.len() <= 1 {
        println!(
            "Seleciona {} vehiculos para competir (formato: 1,5)",
            number_of_cars
        );
        let mut cars_to_compete_string = String::new();

        // Leyendo entrada
        std::io::stdin()
            .read_line(&mut cars_to_compete_string)
            .ok()
            .expect("error");

        // Formateando entrada para procesarla
        let cars_to_compete_input: Vec<i32> = cars_to_compete_string
            .split(",")
            .map(|s| {
                s.trim()
                    .parse()
                    .expect("Tienes que seleccionar los numeros de los vehiculos.")
            })
            .collect();

        // Validando que el numero de vehiculos seleccionados sea el correcto
        if cars_to_compete_input.len() > number_of_cars as usize {
            println!("Solo puedes seleccionar {} vehiculos", number_of_cars);
            continue;
        } else if cars_to_compete_input.len() < number_of_cars as usize {
            println!("Debes seleccionar {} vehiculos", number_of_cars);
            continue;
        }

        // Validando que los vehiculos seleccionados existan
        for car in cars_to_compete_input {
            if car < 0 || car > cars.len() as i32 {
                println!("{} no es un vehiculo valido", car);
                continue;
            }

            let car_to_string = cars[car as usize];
            if !cars.contains(&car_to_string) {
                println!("{} no es un vehiculo valido", car);
                continue;
            }

            cars_to_compete.push(car);
        }
    }

    // Creando objeto para almacenar la distancia recorrida por cada carro
    let mut cars_distance_ran = HashMap::new();
    for car in cars_to_compete {
        cars_distance_ran.insert(car, 0);
    }

    let delay = std::time::Duration::from_millis(750);

    // COLORES
    let red_light = "\x1B[41m   \x1B[0m";
    let yellow_light = "\x1B[43m   \x1B[0m";
    let green_light = "\x1B[42m   \x1B[0m";

    print!("{}[2J", 27 as char);

    // Luz roja
    println!(
        "{}\n{}\n{}",
        red_light.repeat(20),
        red_light.repeat(20),
        red_light.repeat(20)
    );
    std::thread::sleep(delay);

    // Luz amarilla
    print!("{}[2J", 27 as char);
    println!(
        "{}\n{}\n{}",
        yellow_light.repeat(20),
        yellow_light.repeat(20),
        yellow_light.repeat(20)
    );
    std::thread::sleep(delay);

    // Luz verde
    print!("{}[2J", 27 as char);
    println!(
        "{}\n{}\n{}",
        green_light.repeat(20),
        green_light.repeat(20),
        green_light.repeat(20)
    );
    std::thread::sleep(delay);

    // Loop de la carrera
    let mut running = true;
    while running {
        // Cada 450 milisegundos se actualiza la pantalla
        std::thread::sleep(std::time::Duration::from_millis(450));
        print!("{}[2J", 27 as char);

        // Calculando la distancia recorrida por cada carro, y posteriormente renderizando
        for (car, distance) in &mut cars_distance_ran {
            let car_to_string = cars[*car as usize];

            // Creando un texto donde se va a renderizar el carro
            let mut car_string = String::new();

            // Agregando espacios para que el carro se mueva
            for _ in 0..*distance {
                car_string.push_str("ㅤ");
            }

            // Agregando el carro
            car_string.push_str("☁️☁️🚗");

            // Renderizando el carro y la distancia recorrida;
            println!("{}\n{}", car_to_string, car_string);

            // Generando un numero aleatorio para saber que tanto se mueve el carro
            *distance += rand::thread_rng().gen_range(1..4);
            if *distance >= 20 {
                running = false;
                std::thread::sleep(std::time::Duration::from_millis(450));
                break;
            }
        }
    }

    // Seleccionando al ganador
    let mut highest = 0;
    let mut highest_distance = 0;
    for (car, distance) in &cars_distance_ran {
        if distance > &highest_distance {
            highest = *car;
            highest_distance = *distance;
        }
    }

    // Renderizando una bandera
    let flag_frames = vec![
        r#"
                 _____
         _______/     \______
    ___/                    \___
    \                          /
     \                        /
     |\          ____        /
     | \_______/    \_______/
     |
     |
     |
     |
     |
    ---
        "#,
        r#"
                   _____
           _______/      \______
      ___/                    \___
     |\                            /
     | \                          /
     |  \                        /
     |   \          ____        /
     |    \_______/    \_______/
     |
     |   
     |
    ---
        "#,
        r#"
                   _____
           _______/     \______
      ___/                    \___
     |\                          /
     | \                        /
     |  \______          ______/
     |         \________/
     |
     |
     |
     |
    ---
        "#,
    ];

    // Contador de fotogragamas, para renderizar las 3 partes de la bandera
    let mut frame_index = 0;

    loop {
        print!("{}[2J", 27 as char); // Clear the screen

        let flag_frame = flag_frames[frame_index];
        println!("{}", flag_frame);
        println!("¡🥇 {} ha ganado!", cars[highest as usize]);

        // Incrementando el contador
        frame_index += 1;

        // Cuando se llega al final de las fotogragamas, se reinicia el contador
        if frame_index >= flag_frames.len() {
            frame_index = 0;
        }

        // Esperando 500 milisegundos para renderizar el siguiente fotografama de la bandera
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
