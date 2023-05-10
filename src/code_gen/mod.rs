use std::collections::HashSet;
use rand::Rng;
use serde::{Serialize, Deserialize};
use indicatif::{ProgressBar, ProgressStyle};
use rand::distributions::Alphanumeric;
use crate::cli_options::Algorithm;

#[derive(Serialize, Deserialize, Debug)]
pub struct Codes {
    pub codes: Vec<String>,
}

pub fn generate_codes(algorithm: &Algorithm, length: usize, number: usize) -> Result<Codes, &'static str> {
    if length < 1 {
        return Err("Length must be greater than 0");
    }

    let max_possible_codes = match algorithm {
        Algorithm::Alphanumeric => 62usize.pow(length as u32),
        Algorithm::Numeric => 10usize.pow(length as u32),
        Algorithm::Hex => 16usize.pow(length as u32),
    };

    if number > max_possible_codes {
        return Err("The required number of codes is greater than the maximum possible number of codes");
    }

    let mut rng = rand::thread_rng();
    let mut codes = Vec::with_capacity(number);
    let mut unique_codes = HashSet::with_capacity(number);

    let pb = ProgressBar::new(number as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})").expect("Failed to apply template for progress bar")
            .progress_chars("#>-"),
    );

    while unique_codes.len() < number {
        let code = match algorithm {
            Algorithm::Alphanumeric => generate_alphanumeric(&mut rng, length),
            Algorithm::Numeric => generate_numeric(&mut rng, length),
            Algorithm::Hex => generate_hex(&mut rng, length),
        };
        if unique_codes.insert(code.clone()) {
            codes.push(code);
            pb.inc(1);
        }
    }

    pb.finish_and_clear();

    Ok(Codes { codes })
}

fn generate_alphanumeric(rng: &mut impl Rng, length: usize) -> String {
    rng.sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn generate_numeric(rng: &mut impl Rng, length: usize) -> String {
    (0..length)
        .map(|_| rng.gen_range(0..=9))
        .map(|num| std::char::from_digit(num, 10).unwrap())
        .collect()
}

fn generate_hex(rng: &mut impl Rng, length: usize) -> String {
    (0..length)
        .map(|_| rng.gen_range(0..=15))
        .map(|num| std::char::from_digit(num, 16).unwrap())
        .collect()
}
