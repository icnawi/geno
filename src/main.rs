mod code_gen;
mod file_writer;
mod cli_options;

use structopt::StructOpt;
use cli_options::Opt;
use ansi_term::{ANSIGenericString, Color, Style};


fn main() {
    let opt = Opt::from_args();
    println!("Generating codes...");
    match code_gen::generate_codes(&opt.algorithm, opt.length, opt.number) {
        Ok(codes) => {
            let code_generation_success_msg: ANSIGenericString<str> = Style::new().fg(Color::Green).paint("✓");
            let codes_to_json_success_msg: ANSIGenericString<str> = Style::new().fg(Color::Green).paint("✓");

            // Message #1: Codes are generated
            println!("{:<width$} {msg}", code_generation_success_msg, width = code_generation_success_msg.len(), msg = Color::RGB(255,255,255).bold().paint("Codes generated successfully"));
            if let Err(e) = file_writer::write_codes_to_file(&opt.output, &codes.codes, opt.number) {
                eprintln!("{} Failed to write codes to file: {}", Style::new().fg(Color::Red).paint("✗"), e);
            }

            // Message #2: Written to JSON file
            println!("{:<width$} {msg} {pwd}", codes_to_json_success_msg, width = codes_to_json_success_msg.len(), msg = Color::RGB(255,255,255).bold().paint("Codes successfully written to"), pwd = opt.output.display());

        }
        Err(e) => {
            eprintln!("{} Failed to generate codes: {}", Style::new().fg(Color::Red).paint("✗"), e);
        }
    }
}
