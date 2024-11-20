mod build;
use build::BuildCommand;

mod inspect;
use inspect::InspectCommand;

// crates.io
use clap::{
	builder::{
		styling::{AnsiColor, Effects},
		Styles,
	},
	Parser,
};
// self
use crate::prelude::*;

pub trait Run {
	fn run(self) -> Result<()>;
}

/// Cli.
#[derive(Debug, Parser)]
#[command(
	version = concat!(
		env!("CARGO_PKG_VERSION"),
		"-",
		env!("VERGEN_GIT_SHA"),
		"-",
		env!("VERGEN_CARGO_TARGET_TRIPLE"),
	),
	about,
	rename_all = "kebab",
	styles = styles(),
)]
pub struct Cli {
	/// Polkadot Runtime Releaser subcommands.
	#[command(subcommand)]
	subcommand: Subcommand,
}
impl Run for Cli {
	fn run(self) -> Result<()> {
		match self.subcommand {
			Subcommand::Build(build) => build.run(),
			Subcommand::Inspect(inspect) => inspect.run(),
		}
	}
}

#[derive(Debug, Parser)]
enum Subcommand {
	/// Build the polkadot-sdk-based runtime.
	Build(BuildCommand),
	/// Inspect the WASM runtime.
	Inspect(InspectCommand),
}

fn styles() -> Styles {
	Styles::styled()
		.header(AnsiColor::Red.on_default() | Effects::BOLD)
		.usage(AnsiColor::Red.on_default() | Effects::BOLD)
		.literal(AnsiColor::Blue.on_default() | Effects::BOLD)
		.placeholder(AnsiColor::Green.on_default())
}
