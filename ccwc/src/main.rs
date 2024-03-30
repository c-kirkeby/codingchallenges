pub mod cli;

use clap::Parser;
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt, AsyncBufReadExt};
use self::cli::Cli;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Cli::parse();

    let file = File::open(&args.file).await.expect(
        "No such file or directory"
    );
    let reader = io::BufReader::new(file);
    let mut stdout = io::stdout();

    let (mut byte_count, mut line_count, mut word_count) = (0, 0, 0);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        if args.count {
            byte_count += line.as_bytes().len() + 1; // include newline
        }
        if args.length {
            line_count += 1;
        }
        if args.words {
            word_count += line.split_whitespace().count();
        }
    }

    stdout.write_all(format!("\t{line_count}\t{word_count}\t{byte_count}\t{}\n", &args.file).as_bytes()).await?;

    Ok(())
}
