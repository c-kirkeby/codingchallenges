pub mod cli;

use clap::Parser;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use self::cli::Cli;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Cli::parse();

    let file = File::open(&args.file).await?;
    let mut reader = io::BufReader::new(file);
    let mut stdout = io::stdout();

    let mut count = 0;
    let mut buffer = vec![0; 128];

    loop {
        let bytes = reader.read(&mut buffer).await?;

        if bytes == 0 {
            break;
        }

        if args.count {
            count += buffer[..bytes].len();
        }
    }

    stdout.write_all(format!("\t{}\t{}\n", count, &args.file).as_bytes()).await?;

    Ok(())
}
