mod converter;
mod timer;
mod game;

use std::io;
use timer::Timer;
use game::{
    infinite::infinite,
    timed::timed,
    set_number::set_questions
};

fn print_intro() {
    println!("*******************************");
    println!("Olá bem vindo ao jogo das bases");
    println!("*******************************");
}

fn main() {
    let mut timer = Timer::new();

    print_intro();

    'menuloop: loop {
        println!("\nEscolha o modo de jogo:");
        println!("1 - Jogo infinito");
        println!("2 - Definir número de perguntas");
        println!("3 - 5 minutos");
        println!("4 - Sair");

        let mut ans = String::new();
        io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");

        match ans.trim().parse() {
            Ok(1) => infinite(&mut timer),
            Ok(2) => set_questions(&mut timer),
            Ok(3)=> timed(&mut timer),
            Ok(4) => break 'menuloop,
            _ => println!("Opção inválida: {0}", ans)
        }
    }
}
