mod answer;
mod question;

use std::error::Error;
use std::io::{self, BufRead};

use std::env;
use std::fs;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn load_input(max: i32) -> Result<i32, io::Error> {
	let mut input = std::i32::MAX;

	while !(1..=max as i32).contains(&input) {
		input = {
			let mut input = String::new();
			let mut parsed = input.parse::<i32>();

			while let Err(_) = parsed {
				io::stdin().lock().read_line(&mut input)?;
				parsed = input.trim().parse::<i32>();
			}

			parsed.unwrap()
		};
	}

	Ok(input)
}

fn main() -> Result<(), Box<dyn Error>> {
	let tests: Vec<_> = env::args()
		.filter_map(|f| fs::read_to_string(f).ok())
		.map(|s| match question::Question::load(&s) {
			Ok(q) => q,
			Err(e) => panic!("Error on line: {}", e),
		})
		.collect();

	let mut answers: Vec<i32> = Vec::new();

	for test in tests.iter() {
		for question in test.iter().enumerate() {
			println!("\n{}. {}", question.0 + 1, question.1);
			answers.push(load_input(question.1.answers().len() as i32)?);
		}
	}

	let mut stdout = StandardStream::stdout(ColorChoice::Always);

	let mut accum = 0;

	for test in tests.iter() {
		let mut correct_answers = 0;

		for (idx, question) in test.iter().enumerate() {
			stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;

			let choice = (answers[accum] - 1) as usize;
			let correct = question
				.answers()
				.iter()
				.filter(|x| x.correct())
				.next()
				.ok_or("No valid answer")?;

			accum += 1;
			if question.answers()[choice] == *correct {
				print!("{}. {} - ", idx + 1, question.question());
				stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
				println!("{} [OK]", correct);
				correct_answers += 1;
			} else {
				print!("{}. {} - ", idx + 1, question.question());
				stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
				print!("{} [NOPE], ", question.answers()[choice],);
				stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
				println!("correct answer: {}", correct);
			}
		}

		println!(
			"Accuracy: ({}/{}) ({:.2}%)",
			correct_answers,
			test.len(),
			100.0 * correct_answers as f64 / test.len() as f64
		);
	}

	let mut _s = String::new();
	io::stdin().lock().read_line(&mut _s)?;

	Ok(())
}
