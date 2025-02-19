use clap::Parser;

#[derive(Parser)]
#[command(name = "ccwc")]
pub struct Cli {
    #[arg(short = 'c')]
    pub count: bool,

    #[arg(short = 'l')]
    pub length: bool,

    #[arg(short = 'w')]
    pub words: bool,

    #[arg(short = 'm')]
    pub chars: bool,

    #[arg(last = true)]
    pub file: Option<String>
}

