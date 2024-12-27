//! Polkadot Runtime Releaser Docker component.

// std
use std::{
	fmt::Display,
	io::{self, IsTerminal},
};
// self
use crate::{
	prelude::*,
	system::{self, CliArgs},
};

/// Docker run arguments.
#[derive(Debug, Default)]
pub struct RunArgs<'a> {
	image: String,
	envs: Vec<String>,
	volumes: Vec<String>,
	command: &'a [&'a str],
}
impl<'a> RunArgs<'a> {
	const DEFAULT_REPOSITORY: &'static str = "ghcr.io/hack-ink/polkadot-runtime-releaser";
	const PROGRAM: &'static str = "docker";

	#[allow(missing_docs)]
	pub fn new(image_version: String, override_docker_image: Option<String>) -> Self {
		let image = override_docker_image
			.unwrap_or_else(|| format!("{}:{image_version}", Self::DEFAULT_REPOSITORY));

		Self { image, ..Default::default() }
	}

	/// Set the environment variable.
	pub fn with_env<T>(&mut self, key: &str, value: T)
	where
		T: Display,
	{
		self.envs.push(format!("{key}={value}"));
	}

	/// Set the volume.
	pub fn with_volume(&mut self, host: &str, container: &str) {
		self.volumes.push(format!("{host}:{container}"));
	}

	/// Set the command.
	pub fn with_command(&mut self, command: &'a [&'a str]) {
		self.command = command;
	}

	/// Run the command.
	pub fn run(self) -> Result<()> {
		tracing::info!("{self:#?}");

		system::run(Self::PROGRAM, &self.to_cli_args())
	}
}
impl CliArgs for RunArgs<'_> {
	fn to_cli_args(&self) -> Vec<&str> {
		let maybe_it = if io::stdin().is_terminal() { "-it" } else { "-t" };
		let mut args = vec!["run", maybe_it, "--rm"];

		for env in &self.envs {
			args.extend_from_slice(&["-e", env]);
		}
		for volume in &self.volumes {
			args.extend_from_slice(&["-v", volume]);
		}

		args.push(&self.image);
		args.extend_from_slice(self.command);

		args
	}
}

#[test]
fn to_cli_args_should_work() {
	fn assert(override_docker_image: Option<String>) {
		let mut run_args = RunArgs::new("0.0.0".into(), override_docker_image.clone());

		run_args.with_env("FOO", "bar");
		run_args.with_volume("/tmp/host", "/tmp/container");
		run_args.with_command(&["echo", "hello"]);

		let cli_args = run_args.to_cli_args();

		assert_eq!(
			cli_args,
			vec![
				"run",
				"-t",
				"--rm",
				"-e",
				"FOO=bar",
				"-v",
				"/tmp/host:/tmp/container",
				&override_docker_image
					.unwrap_or_else(|| format!("{}:0.0.0", RunArgs::DEFAULT_REPOSITORY)),
				"echo",
				"hello"
			]
		);
	}

	assert(None);
	assert(Some("override".into()));
}
