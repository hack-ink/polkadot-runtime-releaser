// std
use std::{env, fs, path::PathBuf};
// crates.io
use clap::Parser;
// self
use crate::{cli::Run, prelude::*};
use prr_lib::{docker::RunArgs, rust, wasmer::Wasmer};

const WASM_EXT: &str = "wasm";
const WASM_EXT_COMPACT: &str = "compact.wasm";
const WASM_EXT_COMPRESSED: &str = "compact.compressed.wasm";

#[derive(Debug, Parser)]
pub struct BuildCmd {
	/// The target runtime crate to build.
	/// This should be the name of the runtime crate in the <Cargo.toml> file.
	#[arg(value_name = "RUNTIME")]
	runtime: String,
	/// The features to enable for the runtime crate.
	#[arg(long, short, value_name = "FEATURES")]
	features: Option<String>,
	/// Whether to store the compressed runtime only.
	#[arg(long)]
	no_compressed_only: bool,
	/// The toolchain version to use for the build; by default, it is set to <stable>.
	/// This won't take effect if there is a <rust-toolchain.toml> file in the project directory,
	/// and that's the recommended way to specify the toolchain version.
	#[arg(long, short, value_name = "VER", verbatim_doc_comment)]
	toolchain_version: Option<String>,
	/// Image version of the <ghcr.io/hack-ink/polkadot-runtime-releaser>.
	#[arg(long, short = 'v', value_name = "VER", default_value_t = String::from("0.1.7"), conflicts_with = "override_docker_image")]
	image_version: String,
	/// Overwrite the default docker image with the specified one.
	/// Use `docker images` to list the available images on your system.
	#[arg(
		long,
		short = 'i',
		value_name = "REPOSITORY",
		verbatim_doc_comment,
		conflicts_with = "image_version"
	)]
	override_docker_image: Option<String>,
	/// The polkadot-sdk-based project directory; by default, it is set to the current directory.
	#[arg(long, short = 'd', value_name = "PATH")]
	workdir: Option<PathBuf>,
	/// The target directory of the cargo build.
	#[arg(
		long,
		short = 'o',
		value_name = "PATH",
		default_value = "./polkadot-runtime-releaser-output"
	)]
	output_dir: PathBuf,
	/// Whether to cache and use the output of the build.
	/// This is useful in local development.
	#[arg(long)]
	cache_output: bool,
	/// Whether to cache and use the <$HOME/.cargo/registry> registry.
	/// This is useful in local development.
	#[arg(long)]
	cache_registry: bool,
}
impl Run for BuildCmd {
	// ? Remove the created files to keep clean, e.g., `rust-toolchain.toml`.
	fn run(self) -> Result<()> {
		let Self {
			runtime,
			features,
			no_compressed_only,
			toolchain_version,
			image_version,
			override_docker_image,
			workdir,
			output_dir,
			cache_output,
			cache_registry,
		} = self;
		let workdir = workdir.map(|w| w.canonicalize()).unwrap_or_else(env::current_dir)?;

		rust::gen_toolchain_config(toolchain_version, &workdir)?;

		let output_dir = {
			if !output_dir.exists() {
				tracing::info!("creating the output directory {}", output_dir.display());

				fs::create_dir(&output_dir)?;
			}

			output_dir.canonicalize()?
		};
		let container_output_dir =
			format!("/{}", output_dir.file_name().expect("dir must exist").to_string_lossy());
		let mut run_args = RunArgs::new(image_version, override_docker_image);

		run_args.with_env("HOST_UID", users::get_current_uid());
		run_args.with_env("HOST_GID", users::get_current_gid());
		run_args.with_env("CARGO_TARGET_DIR", format!("{container_output_dir}/target"));
		run_args.with_volume(&workdir.to_string_lossy(), "/workdir");
		run_args.with_volume(&output_dir.to_string_lossy(), &container_output_dir);

		let output_target_dir = output_dir.join("target");

		if !cache_output && output_target_dir.exists() {
			tracing::info!(
				"purging previous output target directory {}",
				output_target_dir.display()
			);

			fs::remove_dir_all(&output_target_dir)?;
		}
		if cache_registry {
			let home = env::var("HOME")?;

			run_args.with_volume(&format!("{home}/.cargo/git"), "/root/.cargo/git");
			run_args.with_volume(&format!("{home}/.cargo/registry"), "/root/.cargo/registry");
		}

		let mut cmd = vec!["cargo", "b", "-r", "--locked", "-p", &runtime];

		if let Some(feat) = &features {
			cmd.push("--features");
			cmd.push(feat);
		}

		run_args.with_command(&cmd);
		run_args.run()?;

		let snake_case_rt = runtime.replace("-runtime", "_runtime");
		let output_rt =
			output_target_dir.join("release/wbuild").join(&runtime).join(&snake_case_rt);
		let compressed_wasm = output_rt.with_extension(WASM_EXT_COMPRESSED);
		let ver = Wasmer::load(&compressed_wasm)?.runtime_version(true)?.spec_version;
		let rt_name = format!("{snake_case_rt}-{ver}");

		fs::copy(compressed_wasm, output_dir.join(&rt_name).with_extension(WASM_EXT_COMPRESSED))?;

		if no_compressed_only {
			fs::copy(
				output_rt.with_extension(WASM_EXT_COMPACT),
				output_dir.join(&rt_name).with_extension(WASM_EXT_COMPACT),
			)?;
			fs::copy(
				output_rt.with_extension(WASM_EXT),
				output_dir.join(&rt_name).with_extension(WASM_EXT),
			)?;
		}
		if !cache_output {
			tracing::info!(
				"cleaning up the output target directory {}",
				output_target_dir.display()
			);

			fs::remove_dir_all(&output_target_dir)?;
		}

		Ok(())
	}
}
