use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author,version,about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Ask {
        #[clap(short, long, value_parser, default_value = "default query")]
        query: String,
    },
}
