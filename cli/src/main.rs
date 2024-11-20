//! Polkadot Runtime Releaser CLI.

#![deny(clippy::all, missing_docs, unused_crate_dependencies)]

mod cli;
mod util;

mod prelude {
	pub use anyhow::Result;

	pub(crate) use crate::util;
}

fn main() -> prelude::Result<()> {
	color_eyre::install().unwrap();
	tracing_subscriber::FmtSubscriber::builder().with_max_level(tracing::Level::INFO).init();

	let default_hook = std::panic::take_hook();

	std::panic::set_hook(Box::new(move |p| {
		default_hook(p);

		std::process::abort();
	}));

	cli::Run::run(<cli::Cli as clap::Parser>::parse())?;

	Ok(())
}
