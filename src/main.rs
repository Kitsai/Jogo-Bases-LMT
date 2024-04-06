mod converter;
mod timer;

use std::{convert, default, io::{self, Write}};

use rand::prelude::*;
use converter::*;
use timer::Timer;

fn print_intro() {
    println!("*******************************");
    println!("Olá bem vindo ao jogo das bases");
    println!("*******************************");
}

fn question() -> bool {
    let mut rng = rand::thread_rng();
    let num:u8 = rng.gen_range(0..=255);
    let op: u8 = rng.gen_range(0..=5);
    let mut ans = String::new();

    let result = match op{
        0 => {
            println!("Converta de decimal para binário!");
            print!("O número é {num}: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");
            converter::val_from_binary(&ans) == num
        },
        1 => {
            println!("Converta de decimal para hexadecimal!");
            print!("O número é {num}: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");
            converter::val_from_hex(&ans) == num
        },
        2 => {
            println!("Converta de binário para decimal!");
            print!("O número é {0}: ", convert_to_binary(num));
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");
            val_from_decimal(&ans) == num
        },
        3 => {
            println!("Converta de binário para hexadecimal!");
            print!("O número é {0}: ", convert_to_binary(num));
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");
            val_from_hex(&ans) == num
        },
        4 => {
            println!("Converta de hexadecimal para decimal!");
            print!("O número é {0}: ", convert_to_hex(num));
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");
            val_from_decimal(&ans) == num
        },
        5 => {
            println!("Converta de hexadecimal para binário!");
            print!("O número é {0}: ", convert_to_hex(num));
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");
            val_from_binary(&ans) == num
        }
        _ => {false}
    };

    if result {
        println!("Parabéns você acertou!");
    } else {
        let correct = match op {
            0|5 => convert_to_binary(num),
            1|3 => convert_to_hex(num),
            2|4 => num.to_string(),
            _ => String::new()
        };
        println!("Você errou! O número correto era {0}", correct);
    }

    result  
}

fn main() {
    let mut timer = Timer::new();

    print_intro();

    let mut points: usize = 0;

    'gameloop: loop {
        timer.start();
        let result = question();
        timer.stop();
        println!("Você fez em {0} segundos", timer.get_time_intant().as_secs());
        if result {
            points += 1;
        }

        println!("Quer continuar? (s/n)");
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");
        if ans.trim() != "s" {
            break 'gameloop;
        }
    }
    println!("Você fez {0} pontos", points);
    println!("Demorou {0} segundos", timer.get_time().as_secs());
    println!("Obrigado por jogar!");
}
