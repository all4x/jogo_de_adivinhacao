use rand::Rng;
use std::cmp::Ordering;
use std::{io, u32};

fn main() {
    println!("Advinhe o número!");

    loop {
        println!("Digite seu palpite");

        let numero_secreto = rand::thread_rng().gen_range(1..100);

        let mut palpite = String::new();

        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler a entrada");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você disse: {palpite}");

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito Baixo"),
            Ordering::Greater => println!("Muito Alto"),
            Ordering::Equal => {
                println!("Acertou");
                break;
            }
        };
    }
}
