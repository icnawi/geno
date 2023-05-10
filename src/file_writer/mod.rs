use std::fs::{File, create_dir_all};
use std::io::{BufWriter, Write};
use std::path::Path;
use serde::Serialize;
use indicatif::{ProgressBar, ProgressStyle};

pub fn write_codes_to_file<T: Serialize>(output_path: &Path, codes: &[T], number: usize) -> std::io::Result<()> {
    if let Some(parent) = output_path.parent() {
        create_dir_all(parent)?;
    }

    let file = File::create(output_path)?;
    let mut buffered_writer = BufWriter::new(file);

    let pb = ProgressBar::new(number as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})").expect("Failed to apply template for progress bar")
            .progress_chars("#>-"),
    );

    // Write the opening bracket for the JSON object
    buffered_writer.write_all(b"{\"codes\": [")?;

    // Iterate through the codes and write them to the file
    for (index, code) in codes.iter().enumerate() {
        serde_json::to_writer(&mut buffered_writer, &code)?;

        // Write a comma separator between code items, except for the last item
        if index < codes.len() - 1 {
            buffered_writer.write_all(b",")?;
        }

        // Increment the progress bar
        pb.inc(1);
    }

    // Write the closing brackets for the JSON object and array
    buffered_writer.write_all(b"]}")?;

    // Finish and clear the progress bar
    pb.finish_and_clear();

    // Flush the buffered writer
    buffered_writer.flush()?;

    Ok(())
}
