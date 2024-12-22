mod build;
use build::BuildCmd;

mod cmp;
use cmp::CmpCmd;

mod inspect;
use inspect::InspectCmd;

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
			Subcommand::Build(cmd) => cmd.run(),
			Subcommand::Cmp(cmd) => cmd.run(),
			Subcommand::Inspect(cmd) => cmd.run(),
		}
	}
}

#[derive(Debug, Parser)]
enum Subcommand {
	/// Build the polkadot-sdk-based runtime.
	Build(BuildCmd),
	/// Compare the latest GitHub release's runtime version with the on-chain's.
	Cmp(CmpCmd),
	/// Inspect the WASM runtime.
	Inspect(InspectCmd),
}

fn styles() -> Styles {
	Styles::styled()
		.header(AnsiColor::Red.on_default() | Effects::BOLD)
		.usage(AnsiColor::Red.on_default() | Effects::BOLD)
		.literal(AnsiColor::Blue.on_default() | Effects::BOLD)
		.placeholder(AnsiColor::Green.on_default())
}
