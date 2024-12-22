// std
use std::env;
// crates.io
use clap::Parser;
// self
use crate::{cli::Run, prelude::*};
use prr_lib::{
	api::{Api, ApiConfig},
	runtime::Version,
	wasmer::Wasmer,
};

#[derive(Debug, Parser)]
pub struct CmpCmd {
	/// Remote repository to compare against.
	#[arg(value_name = "REPO")]
	repo: String,
	/// Live node URIs.
	#[arg(value_name = "[URI]")]
	uris: Vec<String>,
}
impl Run for CmpCmd {
	fn run(self) -> Result<()> {
		let Self { repo, uris } = self;
		let github_token = env::var("GITHUB_TOKEN")?;
		let config = ApiConfig { github_token, repo };

		Ok(())
	}
}
