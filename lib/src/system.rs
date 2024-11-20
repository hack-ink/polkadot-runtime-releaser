//! Polkadot Runtime Releaser system component.

// std
use std::process::{self, Command, Stdio};
// self
use crate::prelude::*;

/// Command line arguments.
pub trait CliArgs {
	/// Convert to command line arguments.
	fn to_cli_args(&self) -> Vec<&str>;
}

/// Run a program with arguments.
pub fn run(program: &str, args: &[&str]) -> Result<()> {
	tracing::info!("{program} {}", args.join(" "));

	let mut cmd = Command::new(program);

	cmd.args(args);

	let output =
		cmd.stdin(Stdio::inherit()).stdout(Stdio::inherit()).stderr(Stdio::inherit()).output()?;

	if output.status.success() {
		Ok(())
	} else {
		process::exit(output.status.code().unwrap_or(-1));
	}
}
