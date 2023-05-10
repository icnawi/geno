use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "code-generator", about = "A simple code generator")]
pub struct Opt {
    /// Length of the generated code
    #[structopt(short = "L", long = "length", default_value = "6")]
    pub length: usize,

    /// Generation algorithm
    #[structopt(short = "A", long = "algorithm", default_value = "alphanumeric")]
    pub algorithm: Algorithm,

    /// Number of codes to generate
    #[structopt(short = "N", long = "number", default_value = "1")]
    pub number: usize,

    /// Output file path
    #[structopt(short = "O", long = "output", default_value = "dist/codes.json")]
    pub output: PathBuf,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Algorithm {
    Alphanumeric,
    Numeric,
    Hex,
}

impl std::str::FromStr for Algorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "alphanumeric" => Ok(Algorithm::Alphanumeric),
            "numeric" => Ok(Algorithm::Numeric),
            "hex" => Ok(Algorithm::Hex),
            _ => Err(format!("Invalid algorithm: {}", s)),
        }
    }
}

impl Algorithm {
    #[allow(dead_code)]
    pub fn variants() -> [&'static str; 3] {
        ["alphanumeric", "numeric", "hex"]
    }
}
