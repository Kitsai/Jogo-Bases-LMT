use crate::timer::Timer;
use super::question::{question, bucket::Bucket};
use std::io;

pub fn timed(timer: &mut Timer, bucket: &mut Bucket) {

    println!("Aperte enter para começar");
    io::stdin().read_line(&mut String::new()).expect("Erro ao ler a linha");

    timer.start();
    let mut points: usize = 0;
    let mut questions: usize = 0;   

    while timer.get_time_instant().as_secs() < 300 {
        questions += 1;
        if question(bucket.get_next()) {
            points += 1;
        }
    }
    timer.stop();

    println!("Você fez {0} pontos", points);
    println!("Fez {0} perguntas", questions);
    println!("E demorou em média {0} segundos por questão", 300 / questions as u64);
}