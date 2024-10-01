use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "TaskMaster")]
#[command(about = "Simple task manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        description: String,
        #[arg(short, long)]
        category: Option<String>,
        #[arg(short, long)]
        priority: Option<String>,
    },
    List {
        #[arg(short, long)]
        category: Option<String>,
    },
    Complete {
        id: u32,
    },
    Delete {
        id: u32,
    },
}
