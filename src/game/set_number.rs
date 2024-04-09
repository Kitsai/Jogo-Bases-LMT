use crate::{game::question::question, timer::Timer};
use std::io;

pub fn n_questions(n: usize, timer: &mut Timer) {
    let mut points: usize = 0;
    for _ in 0..n {
        timer.start();
        let result = question();
        timer.stop();

        if result {
            points += 1;
        }

        println!("Você respondeu em {0} segundos", timer.get_time_instant().as_secs());
        io::stdin().read_line(&mut String::new()).expect("Erro ao ler a linha");
    }

    println!("Você fez {0} pontos", points);
    println!("Demorou {0}:{1} minutos no total.", timer.get_time().as_secs()/60, timer.get_time().as_secs()%60);
    println!("E levou em média {0} segundos por questão", timer.get_time().as_secs() / n as u64);
}

pub fn set_questions(timer: &mut Timer) {
    println!("Quantas perguntas você quer responder?");
    let mut ans = String::new();
    io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");
    let n = ans.trim().parse::<usize>().expect("Erro ao converter para número");
    println!("Aperte enter para iniciar o jogo");
    io::stdin().read_line(&mut ans).expect("Erro ao ler a linha");
    n_questions(n, timer);
}