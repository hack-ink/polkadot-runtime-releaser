// self
use crate::{
	prelude::*,
	system::{self, CliArgs},
};

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

	pub fn new(image_version: String, overwrite_docker_image: Option<String>) -> Self {
		let image = overwrite_docker_image
			.unwrap_or_else(|| format!("{}:{image_version}", Self::DEFAULT_REPOSITORY));

		Self { image, ..Default::default() }
	}

	pub fn with_env(&mut self, key: &str, value: &str) {
		self.envs.push(format!("{key}={value}"));
	}

	pub fn with_volume(&mut self, host: &str, container: &str) {
		self.volumes.push(format!("{host}:{container}"));
	}

	pub fn with_command(&mut self, command: &'a [&'a str]) {
		self.command = command;
	}

	pub fn run(self) -> Result<()> {
		tracing::info!("{self:#?}");

		system::run(Self::PROGRAM, &self.to_cli_args())
	}
}
impl<'a> CliArgs for RunArgs<'a> {
	fn to_cli_args(&self) -> Vec<&str> {
		let mut args = vec!["run", "-it", "--rm"];

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
