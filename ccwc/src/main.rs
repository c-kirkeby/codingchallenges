pub mod cli;

use clap::Parser;
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt, AsyncBufReadExt, AsyncRead};
use self::cli::Cli;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Cli::parse();

    let input: Box<dyn AsyncRead + Unpin> = match args.file.as_deref() {
        None => Box::new(io::stdin()),
        Some(file) => Box::new(File::open(file).await?),
    };

    let reader = io::BufReader::new(input);
    let mut stdout = io::stdout();

    let flags = args.count || args.length || args.words || args.chars;
    let (mut byte_count, mut line_count, mut word_count, mut char_count) = (0, 0, 0, 0);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        if !flags || args.count {
            byte_count += line.as_bytes().len() + 1; // include newline
        }
        if !flags || args.length {
            line_count += 1;
        }
        if !flags || args.words {
            word_count += line.split_whitespace().count();
        }
        if args.chars {
            char_count += line.chars().count() + 1; // include newline
        } 
    }

    let output = [line_count, word_count, byte_count, char_count]
        .iter()
        .filter(|x| **x > 0)
        .map(|x| format!("\t{x}"))
        .collect::<Vec<String>>()
        .concat();

    stdout.write_all(format!("{output}\t{}\n", &args.file.unwrap_or("".to_string())).as_bytes()).await?;

    Ok(())
}

