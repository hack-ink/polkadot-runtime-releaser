//! Polkadot Runtime Releaser CLI.

#![deny(clippy::all, missing_docs, unused_crate_dependencies)]

mod cli;
mod docker;
mod system;

mod prelude {
	pub use anyhow::Result;
}

fn main() -> prelude::Result<()> {
	color_eyre::install().unwrap();
	tracing_subscriber::fmt::init();

	let default_hook = std::panic::take_hook();

	std::panic::set_hook(Box::new(move |p| {
		default_hook(p);

		std::process::abort();
	}));

	cli::Run::run(<cli::Cli as clap::Parser>::parse())?;

	Ok(())
}
