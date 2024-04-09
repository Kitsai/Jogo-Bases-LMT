use crate::timer::Timer;
use std::io;
use super::question::question;

pub fn infinite(timer: &mut Timer) {
    let mut points: usize = 0;
    let mut questions: usize = 0;

    println!("Aperte enter para começar");
    io::stdin().read_line(&mut String::new()).expect("Erro ao ler a linha");

    'gameloop: loop {
        questions += 1;

        timer.start();
        let result = question();
        timer.stop();
        println!("Você respondeu em {0} segundos", timer.get_time_instant().as_secs());
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
    println!("Demorou {0}:{1} minutos no total.", timer.get_time().as_secs()/60, timer.get_time().as_secs()%60);
    println!("E levou em média {0} segundos por questão", timer.get_time().as_secs() / questions as u64);
}