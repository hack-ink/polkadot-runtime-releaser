// std
use std::{env, fs, path::PathBuf};
// crates.io
use clap::Parser;
// self
use crate::{
	cli::{InspectCmd, Run},
	prelude::*,
};
use prr_lib::{docker::RunArgs, rust, wasmer::Wasmer};

const WASM_EXT: &str = "wasm";
const WASM_EXT_COMPACT: &str = "compact.wasm";
const WASM_EXT_COMPRESSED: &str = "compact.compressed.wasm";

#[derive(Debug, Parser)]
pub struct BuildCmd {
	/// The target runtime to build.
	///
	/// This should be the name of the runtime crate in the <Cargo.toml> file.
	#[arg(value_name = "RUNTIME", verbatim_doc_comment)]
	runtime: String,
	/// The features to enable for the runtime.
	#[arg(long, short, value_name = "FEATURES", verbatim_doc_comment)]
	features: Option<String>,
	/// Whether to store the compressed runtime only.
	#[arg(long, verbatim_doc_comment)]
	no_compressed_only: bool,
	/// Whether to generate the digest file for the runtime.
	#[arg(long, verbatim_doc_comment)]
	no_digest: bool,
	/// The toolchain version to use for the build; by default, it is set to <stable>.
	///
	/// This won't take effect if there is a <rust-toolchain.toml> file in the project directory,
	/// and that's the recommended way to specify the toolchain version.
	#[arg(long, short, value_name = "VER", verbatim_doc_comment)]
	toolchain_ver: Option<String>,
	/// Image version of the <ghcr.io/hack-ink/polkadot-runtime-releaser>.
	#[arg(
		long,
		short = 'v',
		value_name = "VER",
		default_value_t = String::from("0.2.0"),
		verbatim_doc_comment,
		conflicts_with = "override_docker_image"
	)]
	image_version: String,
	/// Overwrite the default docker image with the specified one.
	///
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
	#[arg(long, short = 'd', value_name = "PATH", verbatim_doc_comment)]
	workdir: Option<PathBuf>,
	/// The target directory of the cargo build.
	#[arg(
		long,
		short = 'o',
		value_name = "PATH",
		default_value = "./polkadot-runtime-releaser-output",
		verbatim_doc_comment
	)]
	output_dir: PathBuf,
	/// Whether to cache and use the output of the build.
	/// This is useful in local development.
	#[arg(long, verbatim_doc_comment)]
	cache_output: bool,
	/// Whether to cache and use the <$HOME/.cargo/registry> registry.
	/// This is useful in local development.
	#[arg(long, verbatim_doc_comment)]
	cache_registry: bool,
}
impl Run for BuildCmd {
	// ? Remove the created files to keep clean, e.g., `rust-toolchain.toml`.
	fn run(self) -> Result<()> {
		let Self {
			runtime,
			features,
			no_compressed_only,
			no_digest,
			toolchain_ver,
			image_version,
			override_docker_image,
			workdir,
			output_dir,
			cache_output,
			cache_registry,
		} = self;
		let workdir = workdir.map(|w| w.canonicalize()).unwrap_or_else(env::current_dir)?;

		rust::gen_toolchain_config(toolchain_ver, &workdir)?;

		let output_dir = {
			if !output_dir.exists() {
				tracing::info!("creating the output directory {output_dir:?}");

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
			tracing::info!("purging previous output target directory {output_target_dir:?}",);

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

		// https://github.com/paritytech/polkadot-sdk/blob/ca7817922148c1e6f6856138998f7135f42f3f4f/substrate/utils/wasm-builder/src/wasm_project.rs#L502.
		let snake_case_name = runtime.replace("-", "_");
		let output_rt =
			output_target_dir.join("release/wbuild").join(&runtime).join(&snake_case_name);
		let compressed_rt = output_rt.with_extension(WASM_EXT_COMPRESSED);

		tracing::info!("loading {compressed_rt:?}");

		let wasmer = Wasmer::load(&compressed_rt)?;
		let ver = wasmer.runtime_version(true)?.spec_version;
		let rt_prefix = output_dir.join(format!("{snake_case_name}-{ver}"));

		util::copy(&compressed_rt, &rt_prefix.with_extension(WASM_EXT_COMPRESSED))?;

		if no_compressed_only {
			util::copy(
				&output_rt.with_extension(WASM_EXT_COMPACT),
				&rt_prefix.with_extension(WASM_EXT_COMPACT),
			)?;
			util::copy(&output_rt.with_extension(WASM_EXT), &rt_prefix.with_extension(WASM_EXT))?;
		}
		if !no_digest {
			let digest_path = rt_prefix.with_extension("json");

			tracing::info!("generating {digest_path:?}");

			let digest = InspectCmd::new(compressed_rt, false, false, false).inspect(wasmer)?;

			fs::write(&digest_path, digest)?;
		}
		if !cache_output {
			tracing::info!("cleaning up the output target directory {output_target_dir:?}",);

			fs::remove_dir_all(&output_target_dir)?;
		}

		Ok(())
	}
}
