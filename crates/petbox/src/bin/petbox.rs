#[macro_use]
extern crate log;
use clap::{Args, Parser, Subcommand};
use petbox::config::Config;
use std::path::Path;
#[cfg(debug_assertions)]
const DEBUG_ENV: bool = true;

#[derive(Parser)]
#[command(author, version, about, long_about = None, disable_help_flag(true))]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command()]
    /// Create new petbox rootfs container
    Create(Create),

    #[command(subcommand)]
    /// Low-level container runtime
    ///
    /// Run a process in a petbox container with new namespace
    /// This sub-command will always create a new namespace, so you may not
    /// want to use this command directly
    Wrap(Wrap),

    #[command()]
    /// Start a container and put it in background
    Start(Start),

    #[command(subcommand)]
    /// Low-level container monitor utility
    /// 
    /// You may not want to use this command directly
    Cmon(Cmon),

    #[command()]
    /// Run a program inside a a petbox container
    ///
    /// This sub-command will try to use existent namespace,
    /// and will start one when appropriate
    Exec(Exec),
}

#[derive(Subcommand)]
enum Wrap {

    
}

#[derive(Subcommand)]
enum Cmon {
    
}

#[derive(Args)]
struct Create {
    #[arg(short, long)]
    /// Name of the container
    name: String,

    #[arg(short, long)]
    /// Image to use for the container
    source: String,

    #[arg(short, long)]
    /// Sharing home directory with the container
    ///
    /// This will cause behaviour similar to distrobox
    // TODO
    home: bool,

    #[arg(long)]
    /// Enter the namespace without extracting rootfs
    enter_ns: bool,

    #[arg(long,action = clap::ArgAction::Help)]
    /// Show this message
    help: (),
}

#[derive(Args)]
struct Attach {
    #[arg(short, long)]
    /// Name of the container
    name: String,

    #[arg(long)]
    /// Show this message
    help: bool,
}

#[derive(Args)]
struct Run {
    #[arg(short, long)]
    /// Name of the container
    name: String,

    #[arg(long,action = clap::ArgAction::Help)]
    /// Show this message
    help: (),

    #[arg(last = true)]
    /// Command to execute
    command: Vec<String>,
}

#[derive(Args)]
struct Start {
    #[arg(short, long)]
    /// Name of the container
    name: String,
}

#[derive(Args)]
struct Exec {
    #[arg(short, long)]
    /// Name of the container
    name: String,

    #[arg(long,action = clap::ArgAction::Help)]
    /// Show this message
    help: (),
}

fn main() {
    let mut logger: env_logger::Builder;
    if DEBUG_ENV {
        logger =
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"));
        warn!("You are using dev build of petbox compiled without optimization.")
    } else {
        logger =
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"));
    }
    logger.format_timestamp(None).init();
    //let mut cmd = Cli::command_for_update();
    let cli = Cli::parse();
    match &cli.command {
        Commands::Create(opt) => {
            let config = Config::build();
            let root_path = config.get_container_rootfs(&opt.name);
            todo!()
        }
        Commands::Wrap(opt) => {
            todo!()
        }
        Commands::Exec(_) => todo!(),
        Commands::Start(_) => todo!(),
        Commands::Cmon(_) => todo!(),
    }
}
