mod converter;
mod timer;
mod game;

use std::{io, thread::sleep, time::Duration};
use timer::Timer;
use game::{
    infinite::infinite,
    timed::timed,
    set_number::set_questions,
    question::bucket::{Bucket, BucketTypes}
};

fn print_intro() {
    println!("*******************************");
    println!("Olá bem vindo ao jogo das bases");
    println!("*******************************");
}

fn main() {
    let mut timer = Timer::new();
    let mut bucket = Bucket::new(BucketTypes::Multiple(2));

    print_intro();

    'menuloop: loop {
        println!("\nEscolha o modo de jogo:");
        println!("1 - Jogo infinito");
        println!("2 - Definir número de perguntas");
        println!("3 - 5 minutos");
        println!("4 - change question types");
        println!("5 - Sair");

        let mut ans = String::new();
        io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");

        bucket.reset();

        match ans.trim().parse() {
            Ok(1) => infinite(&mut timer,&mut bucket),
            Ok(2) => set_questions(&mut timer, &mut bucket),
            Ok(3) => timed(&mut timer, &mut bucket),
            Ok(4) => change_bucket(&mut bucket),
            Ok(5) => break 'menuloop,
            _ => println!("Opção inválida: {0}", ans)
        }
    }
    println!("Até mais!");
    sleep(Duration::from_secs(1))
}

fn change_bucket(bucket: &mut Bucket) {
    println!("\nEscolha o tipo das perguntas:");
    println!("1 - Padrão (Todas aparecem igualmente)");
    println!("2 - Customizado (Escolha as perguntas)");

    let mut ans = String::new();
    io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");

    match ans.trim().parse() {
        Ok(1) => {
            println!("\nQuantas de cada pergunta deve ter? (Quanto menor o número, mais vezes aparece)");
            let mut ans = String::new();
            io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");
            match ans.trim().parse() {
                Ok(n) => {
                    bucket.change_type(BucketTypes::Multiple(n));
                },
                _ => println!("Valor inválido: {0}", ans)
            }
        },
        Ok(2) => {
            println!("Perguntas disponíveis:");
            println!("0 - Decimal para binário");
            println!("1 - Decimal para hexadecimal");
            println!("2 - Binário para decimal");
            println!("3 - Binário para hexadecimal");
            println!("4 - Hexadecimal para decimal");
            println!("5 - Hexadecimal para binário");
            println!("\nDigite as perguntas separadas por espaço:");
            let mut ans = String::new();
            io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");
            let questions: Vec<u8> = ans.trim().split(" ").map(|x| x.parse().unwrap()).collect();
            bucket.change_type(BucketTypes::Custom(questions));
        },
        _ => println!("Opção inválida: {0}", ans)
    }
}
