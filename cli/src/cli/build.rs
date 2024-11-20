// std
use std::{env, fs, path::PathBuf};
// crates.io
use clap::Parser;
use nix::unistd;
// self
use crate::{cli::Run, docker::RunArgs, prelude::*};

#[derive(Debug, Parser)]
pub struct BuildCommand {
	/// The polkadot-sdk-based project directory; by default, it is set to the current directory.
	#[arg(long, short = 'd', value_name = "PATH")]
	workdir: Option<String>,
	/// Image version of the <ghcr.io/hack-ink/polkadot-runtime-releaser>.
	#[arg(long, short = 'v', value_name = "VER", default_value_t = String::from("latest"), conflicts_with = "overwrite_docker_image")]
	image_version: String,
	/// Overwrite the default docker image with the specified one.
	/// Use `docker image ls` to list the available images on your system.
	#[arg(
		long,
		short = 'i',
		value_name = "REPOSITORY",
		verbatim_doc_comment,
		conflicts_with = "image_version"
	)]
	overwrite_docker_image: Option<String>,
	/// The target directory of the cargo build.
	#[arg(long, short = 'o', value_name = "PATH", default_value_t = String::from("./polkadot-runtime-releaser-output"))]
	output_dir: String,
	/// Whether to cache and use the output of the build.
	#[arg(long)]
	cache_output: bool,
	/// Whether to cache and use the <~/.cargo/registry> registry.
	#[arg(long)]
	cache_registry: bool,
	/// The target runtime crate to build.
	/// This should be the name of the runtime crate in the <Cargo.toml> file.
	#[arg(value_name = "RUNTIME")]
	runtime: String,
}
impl Run for BuildCommand {
	fn run(self) -> Result<()> {
		let Self {
			workdir,
			image_version,
			overwrite_docker_image,
			output_dir,
			cache_output,
			cache_registry,
			runtime,
		} = self;
		let workdir = if let Some(workdir) = workdir {
			PathBuf::from(workdir).canonicalize()?
		} else {
			env::current_dir()?
		};
		let output_dir = {
			let d = PathBuf::from(&output_dir);

			if !d.exists() {
				tracing::info!("creating the output directory {}", d.display());

				fs::create_dir(&d)?;
			}

			d.canonicalize()?
		};
		let container_output_dir =
			format!("/{}", output_dir.file_name().expect("dir must exist").to_string_lossy());
		let mut run_args = RunArgs::new(image_version, overwrite_docker_image);

		run_args.with_env("GID_OVERRIDE", &unistd::getgid().as_raw().to_string());
		run_args.with_env("UID_OVERRIDE", &unistd::getuid().as_raw().to_string());
		run_args.with_env("CARGO_TARGET_DIR", &container_output_dir);
		run_args.with_volume(&workdir.to_string_lossy(), "/workdir");

		if cache_output {
			run_args.with_volume(&output_dir.to_string_lossy(), &container_output_dir);
		}
		if cache_registry {
			let home = env::var("HOME")?;

			run_args.with_volume(&format!("{home}/.cargo/git"), "/root/.cargo/git");
			run_args.with_volume(&format!("{home}/.cargo/registry"), "/root/.cargo/registry");
		}

		let cmd = ["cargo", "b", "-r", "--locked", "-p", &runtime];

		run_args.with_command(&cmd);
		run_args.run()?;

		Ok(())
	}
}
