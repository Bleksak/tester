use crate::answer::Answer;
use rand::seq::SliceRandom;

const QUESTION: &str = "[Q]";
const ANSWER: &str = "[A]";
const CORRECT: &str = "[C]";

pub struct Question {
    question: String,
    answers: Box<[Answer]>,
}

impl Question {
    pub fn new(question: String, answers: Box<[Answer]>) -> Self {
        Self { question, answers }
    }

    pub fn question(&self) -> &str {
        &self.question
    }

    pub fn answers(&self) -> &[Answer] {
        &self.answers
    }

    pub fn load(txt: &str) -> Result<Box<[Question]>, usize> {
        let mut questions: Vec<Question> = Vec::new();

        let mut current_question: Option<&str> = None;
        let mut current_answers: Vec<Answer> = Vec::new();
        let mut rng = rand::thread_rng();

        for (idx, line) in txt.lines().enumerate() {
            let split: Vec<_> = line.splitn(2, " ").collect();
            if split.len() < 2 {
                return Err(idx);
            }

            match split[0] {
                QUESTION => {
                    if let Some(question) = current_question {
                        let mut boxed = current_answers.clone().into_boxed_slice();
                        boxed.shuffle(&mut rng);
                        questions.push(Self::new(question.to_string(), boxed));
                        current_answers.clear();
                    }

                    current_question = Some(split[1]);
                }
                ANSWER => {
                    current_answers.push(Answer::new(split[1].to_string(), false));
                }

                CORRECT => {
                    current_answers.push(Answer::new(split[1].to_string(), true));
                }

                _ => {
                    return Err(idx);
                }
            }
        }

        if let Some(question) = current_question {
            let mut boxed = current_answers.clone().into_boxed_slice();
            boxed.shuffle(&mut rng);
            questions.push(Self::new(question.to_string(), boxed));
            current_answers.clear();
        }

        let mut boxed = questions.into_boxed_slice();
        boxed.shuffle(&mut rng);
        Ok(boxed)
    }
}

impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Err(e) = writeln!(f, "{}\n", self.question()) {
            return Err(e);
        }

        for item in self.answers().iter().enumerate() {
            if let Err(e) = writeln!(f, "{}) {}", item.0 + 1, item.1) {
                return Err(e);
            }
        }

        Ok(())
    }
}
