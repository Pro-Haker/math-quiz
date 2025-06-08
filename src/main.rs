use std::io;
use rand::seq::SliceRandom;

use crate::questions::QUESTIONS;
mod questions;

fn main() {
    let mut questions = QUESTIONS.iter().map(|v| v).collect::<Vec<_>>();
    questions.shuffle(&mut rand::rng());
    let score = questions[0..3]
        .iter()
        .fold(0, |v, q| v + if q.ask() { 1 } else { 0 });

    println!("Thine score: {score}/3");
}

struct Question {
    question: &'static str,
    answers: &'static [&'static str],
}

impl Question {
    fn ask(&self) -> bool {
        let mut answer = String::new();

        println!("{}", self.question);

        io::stdin()
            .read_line(&mut answer)
            .expect("the code sux so excuse me mister, me failed riding da line");

        let result = self.answers.contains(&answer.trim());

        if result {
            println!("noice :3");
        } else {
            println!("bruh, dats not right")
        }

        result
    }
}
