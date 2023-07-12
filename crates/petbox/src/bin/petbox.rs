#[macro_use]
extern crate log;
use clap::{Args, Parser, Subcommand};
use petbox::config::Config;
use petbox::container::{self, Container};
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

    #[command()]
    /// Attach terminal to a running petbox container
    Attach(Attach),

    #[command()]
    /// Start petbox container
    Start(Start),

    #[command()]
    /// Run program inside a running petbox container
    Exec(Exec),
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
struct Start{
    #[arg(short, long)]
    /// Name of the container
    name: String,

    #[arg(short, long)]
    /// Command to execute
    command: Vec<String>,

    #[arg(long,action = clap::ArgAction::Help)]
    /// Show this message
    help: (),
}

#[derive(Args)]
struct Exec{
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
            let root = container::Rootfs::new(&root_path);
            info!("Creating conatiner...");
            trace!("path:{:?},tar_file:{:?}", root_path, opt.source);
            match &opt.enter_ns {
                true => {
                    match root.install_rootfs_enter_ns("/bin/bash") {
                        Ok(_) => {},
                        Err(e) => error!("Command failed: {e}"),
                    };
                }
                false => {
                    match root.install_rootfs_from_tar(Path::new(&opt.source)) {
                        Ok(_) => {},
                        Err(e) => { error!("Failed to extract rootfs: {e}") },
                    };
                }
            }
        }
        Commands::Attach(opt) => {
            info!("Attach to `{}`",opt.name);
            todo!()
        }
        Commands::Start(opt) => {
            info!("Starting `{}`, `{:?}`",opt.name,opt.command);
            let config = Config::build();
            let root_path = config.get_container_rootfs(&opt.name);
            let mut cbox = Container::new(&root_path);
            cbox.start(&opt.command[0], &opt.command[1..]);
        },
        Commands::Exec(_) => todo!(),
    }
}
