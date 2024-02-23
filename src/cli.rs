//! https://docs.rs/clap/latest/clap/

use clap::{ArgAction, Args, Parser, Subcommand};
use log::{debug, trace, LevelFilter};

/// The subcommand handler.
/// If no subcommand is provided, the handler will short to the logic for the default command.
///
/// This struct probably doesn't need to change, make changes to `Subcommands` and the individual
/// subcommands instead.
#[derive(Parser, Debug)]
#[command(name = "creative_coding")]
#[command(bin_name = "creative_coding")]
#[clap(about = "creative_coding cli")]
#[command(author, version)]
#[command(propagate_version = true)]
pub struct MyCli {
  #[command(subcommand)]
  subcommands:   Option<Subcommands>,
  /// Set the verbosity. Use -v for DEBUG, -vv for TRACE. None for INFO.
  #[arg(long = "verbose", short = 'v', action = ArgAction::Count)]
  pub verbosity: u8,
}

impl MyCli {
  pub fn handle(&self) {
    match &self.subcommands {
      Some(subcommands) => subcommands.handle(),
      None => self.handle_default(),
    }
  }

  /// in decreasing order of priority:
  /// if verbosity is specified from command line, e.g. `-v` or `-vv`, use that
  /// if a `RUST_LOG` env var is set, use that
  /// else, use ;FO
  pub fn log_level(&self) -> LevelFilter {
    if self.verbosity > 0 {
      match self.verbosity {
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
      }
    } else if let Ok(s) = std::env::var("RUST_LOG") {
      s.parse().expect("RUST_LOG environment invalid value")
    } else {
      LevelFilter::Info
    }
  }

  /// The default command: what to do if no subcommand is provided
  fn handle_default(&self) { trace!("handle default") }
}

/// CLI parser with subcommands
/// The subcommands for this CLI.
/// Add subcommands as demonstrated.
#[derive(Debug, Subcommand)]
enum Subcommands {
  SayHello(SayHello),
  First(First),
  Window(Window),
  Coordinates(Coordinates),
}

impl Subcommands {
  /// delegate handling to each subcommand
  pub fn handle(&self) {
    trace!("handling subcommands...");
    match self {
      Subcommands::SayHello(c) => c.handle(),
      Subcommands::First(c) => c.handle(),
      Subcommands::Window(c) => c.handle(),
      Subcommands::Coordinates(c) => c.handle(),
    }
  }
}

// test with:
// cargo run -- say-hello --hello
/// An example subcommand
#[derive(Parser, Debug)]
struct SayHello {
  /// example
  #[arg(long = "hello")]
  pub hello_world: bool,
}

impl SayHello {
  pub fn handle(&self) {
    if self.hello_world {
      println!("hello world!");
    }
  }
}

#[derive(Parser, Debug)]
struct First;
impl First {
  pub fn handle(&self) {
    // crate::nannou::first::first();
    crate::nannou::first::sketch()
  }
}


#[derive(Parser, Debug)]
struct Window;

impl Window {
  pub fn handle(&self) {
    crate::nannou::window::draw()
  }
}


#[derive(Parser, Debug)]
struct Coordinates;

impl Coordinates {
  pub fn handle(&self) {
    crate::nannou::coordinates::draw()
  }
}