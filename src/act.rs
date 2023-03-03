use std::ffi::OsString;
use clap::Parser;

pub mod go;
pub mod actor;
pub mod snap;
pub mod version;
//mod misc;

#[derive(Debug, Parser)]
#[command(about,long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Parser)]
pub enum Commands {

    #[command(about = "Ferris action!")]
    Go,
    #[command(about = "snap from animation")]
    Snap,

    #[command(external_subcommand)]
    External(Vec<OsString>),
}

pub fn show() {
    let _guard = clia_tracing_config::build()
        .filter_level("info") //fatal,error,warn,info,debug
        .with_ansi(true)
        .to_stdout(false)
        .directory("./log")
        .file_name("debug.log")
        .rolling("daily")
        .init();
    log::debug!("src/pose/mod:{:?}", _guard);

    let args = Cli::parse();

    match args.command {

        Commands::Go => {
            println!("Calling Anima");
            go::act();
        }
        Commands::Snap => {
            println!("Snap a randomly Ferris ASCII-art pose as Markdown:\n");
            snap::act();

        }
        Commands::External(args) => {
            println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
        }
    }
}
