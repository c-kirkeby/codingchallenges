use clap::Parser;

#[derive(Parser)]
#[command()]
pub struct Cli {
    #[arg(short = 'c')]
    pub count: bool,

    #[arg(short = 'l')]
    pub length: bool,

    #[arg(last = true)]
    pub file: String
}

