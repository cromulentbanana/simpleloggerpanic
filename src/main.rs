use log;
use log::LevelFilter::{Debug, Error, Info, Trace};
use simple_logger::SimpleLogger;
use structopt::StructOpt;

mod webapp;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "logger fail",
    about = "A short description of this project.",
    version = concat!(env!("CARGO_PKG_VERSION"), concat!("_", env!("GIT_SHORT_HASH")))
)]
struct Options {
    /// Suppress non-error messages
    #[structopt(short)]
    quiet: bool,

    /// Increase logging verbosity
    #[structopt(short, parse(from_occurrences))]
    verbosity: usize,

    /// Example optional boolean flag
    //#[structopt(short, long)]
    //some_flag: Option<bool>,

    /// Artificial Delay added to NDO response
    #[structopt(short, long, default_value = "5", env = "ARTIFICIAL_DELAY")]
    delay: u16,

    /// Webserver Port Number
    #[structopt(short = "p", long, default_value = "8080", env = "BIND_PORT")]
    webserver_port: u16,

    /// Webserver IP Address
    #[structopt(short = "b", long, default_value = "127.0.0.1", env = "BIND_ADDRESS")]
    webserver_bind_address: std::net::IpAddr,
}

fn main() {
    let args = Options::from_args();
    let level = match args.quiet {
        true => Error,
        false => vec![Info, Debug, Trace][(args.verbosity).min(2)],
    };

    SimpleLogger::new()
        .with_level(level)
        .init()
        .expect("SimpleLogger instantiation failed.");

    log::info!("Logging with level {}", level);

    webapp::main(args.webserver_bind_address, args.webserver_port, args.delay).unwrap();
}
