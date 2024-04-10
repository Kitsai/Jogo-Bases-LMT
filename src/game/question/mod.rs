pub mod bucket;

use std::io::{self, Write};

use rand::prelude::*;
use crate::converter::*;

pub fn question(op: u8) -> bool {
    let mut rng = rand::thread_rng();
    let num:u8 = rng.gen_range(0..=255);
    let mut ans = String::new();

    let result = match op{
        0 => {
            println!("Converta de decimal para binário!");
            print!("O número é {num}: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");
            match val_from_binary(&ans) {
                Ok(val) => val == num,
                _ => false
            }
        },
        1 => {
            println!("Converta de decimal para hexadecimal!");
            print!("O número é {num}: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");
            match val_from_hex(&ans) {
                Ok(val) => val == num,
                _ => false
            }
        },
        2 => {
            println!("Converta de binário para decimal!");
            print!("O número é {0}: ", convert_to_binary(num));
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");
            match val_from_decimal(&ans) {
                Ok(val) => val == num,
                _ => false
            }
        },
        3 => {
            println!("Converta de binário para hexadecimal!");
            print!("O número é {0}: ", convert_to_binary(num));
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");
            match val_from_hex(&ans) {
                Ok(val) => val == num,
                _ => false
            }
        },
        4 => {
            println!("Converta de hexadecimal para decimal!");
            print!("O número é {0}: ", convert_to_hex(num));
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");
            match val_from_decimal(&ans) {
                Ok(val) => val == num,
                _ => false
            }
        },
        5 => {
            println!("Converta de hexadecimal para binário!");
            print!("O número é {0}: ", convert_to_hex(num));
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");
            match val_from_binary(&ans) {
                Ok(val) => val == num,
                _ => false
            }
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
