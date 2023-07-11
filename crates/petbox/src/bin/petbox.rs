#[macro_use]
extern crate log;
use std::path::Path;
use clap::{ Args, CommandFactory, Parser, Subcommand};
use petbox::container;
use petbox::config::Config;
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
    #[command(disable_help_flag(true))]
    /// Create new petbox rootfs container
    Create(Create),

    #[command(disable_help_flag(true))]
    /// Attach terminal to a running petbox container
    Attach(Attach),
}

#[derive(Args)]
struct Create {
    #[arg()]
    /// Name of the container
    name: String,

    #[arg()]
    /// Image to use for the container
    source: String,

    #[arg(short, long)]
    /// Sharing home directory with the container
    ///
    /// This will cause behaviour similar to distrobox
    // TODO
    home: bool,

    #[arg(long,action = clap::ArgAction::Help)]
    /// Show this message
    help: (),

    #[arg(long)]
    /// Run without acutally modify on-disk file
    dry_run: bool,

    #[arg(long)]
    /// Enter the namespace without extracting rootfs
    enter_ns: bool,
}

#[derive(Args)]
struct Attach {
    #[arg(short, long)]
    /// Name of the container
    name: Option<String>,

    #[arg(long)]
    /// Show this message
    help: bool,
}

fn opt_help(name: &str) {
    let mut cmd = Cli::command_for_update();
    let cmd = cmd.find_subcommand_mut(name).unwrap();
    //let mut cmd = cmd.disable_help_flag(true);
    cmd.set_bin_name(format!("petbox {name}"));
    cmd.print_help().unwrap();
    println!();
    println!("Use `petbox help {name}` for detailed help message");
    std::process::exit(0);
}

fn main() {
    let mut logger: env_logger::Builder;
    if DEBUG_ENV {
        logger = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"));
        warn!("You are using dev build of petbox compiled without optimization.")
    } else {
        logger= env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"));
    }
    logger.format_timestamp(None).init();
    //let mut cmd = Cli::command_for_update();
    let cli = Cli::parse();
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Create(opt) => {
            let config = Config::build();
            let path = config.get_container_rootfs(&opt.name);
            container::install_rootfs(&path,Path::new(&opt.source),opt.dry_run,opt.enter_ns);
        }
        Commands::Attach(opt) => {
            println!("run")
        }
    }
}
