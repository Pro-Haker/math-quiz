use crate::Question;

pub const QUESTIONS: &[Question] = &[
    Question {
        question: "x^2 + 3x - 10 = 0, x = ? (x1, x2)",
        answers: &["-5, 2", "2, -5"],
    },
    Question {
        question: "What is the remainder when 53 x 19 is divided by 6?",
        answers: &["5", "five"],
    },
    Question {
        question: "You roll two 6-sided dice. What is the probability the total is at least 10?",
        answers: &["1/6", "one sixth"],
    },
    Question {
        question: "8 log16(2) = ?",
        answers: &["2", "two"],
    },
    Question {
        question: "y = 2x + 1, x + y = 10, x, y = ? (x, y)",
        answers: &["3, 7"],
    },
];
