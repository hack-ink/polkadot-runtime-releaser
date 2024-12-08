//! Polkadot Runtime Releaser API component.

// crates.io
use reqwew::{
	error::Error as RError,
	reqwest::{header::USER_AGENT, Client, Method, Url},
	Http, Response,
};
use serde::{de::Error as DeError, Deserialize, Deserializer};
use sp_version::RuntimeVersion;
use subrpcer::state;
// self
use crate::prelude::*;

/// API.
#[derive(Debug)]
pub struct Api {
	client: Client,
	config: ApiConfig,
}
impl Api {
	#[allow(missing_docs)]
	pub fn new(config: ApiConfig) -> Self {
		Self { client: Client::new(), config }
	}

	async fn runtimes(&self) -> Result<Vec<Runtime>> {
		let mut runtimes = Vec::new();

		for rt_config in &self.config.runtime_configs {
			for endpoint in &rt_config.rpc_endpoints {
				match self
					.client
					.request_with_retries(
						self.client
							.request(Method::POST, endpoint.clone())
							.json(&state::get_runtime_version(0, None::<()>))
							.build()
							.map_err(RError::Reqwest)?,
						3,
						50,
					)
					.await
				{
					Ok(resp) =>
						if let Ok(resp) = resp.json::<RpcResponse<RuntimeVersion>>() {
							runtimes.push(Runtime {
								name: rt_config.name.clone(),
								on_chain_version: resp.result.spec_version,
								ok: true,
							});

							break;
						},
					Err(_e) => {
						continue;
					},
				}
			}

			if let Some(rt) = runtimes.last() {
				if rt.name != rt_config.name {
					runtimes.push(Runtime {
						name: rt_config.name.clone(),
						on_chain_version: 0,
						ok: false,
					});
				}
			}
		}

		Ok(runtimes)
	}

	// async fn runtimes(&self) -> Result<()> {
	// 	let releases = self.releases().await.unwrap();
	// 	let mut remains = (1_u8 << self.config.runtime_configs.len()) - 1;

	// 	'outer: for r in releases {
	// 		if remains == 0 {
	// 			break;
	// 		}

	// 		for (i, rt_config) in self.config.runtime_configs.iter().enumerate() {
	// 			if (remains & (1 << i)) != 0 {
	// 				if let Some(asset) = r.assets.iter().find(|asset| {
	// 					let url = &asset.browser_download_url;

	// 					if let Some(caps) = rt_config.version_regex.captures(url) {
	// 						println!("Found version {} at URL: {}", &caps[1], url);

	// 						remains &= !(1 << i);

	// 						true
	// 					} else {
	// 						false
	// 					}
	// 				}) {
	// 					if remains == 0 {
	// 						break 'outer;
	// 					}
	// 				}
	// 			}
	// 		}
	// 	}
	// }

	async fn releases(&self) -> Result<Vec<Release>> {
		let releases = self
			.client
			.request_with_retries(
				self.client
					.request(Method::GET, self.config.releases())
					.header(USER_AGENT, "polkadot-runtime-releaser")
					.bearer_auth(&self.config.github_token)
					.build()
					.map_err(RError::Reqwest)?,
				3,
				50,
			)
			.await?
			.json::<Vec<Release>>()?;

		Ok(releases)
	}
}

/// API configuration.
#[derive(Debug, Deserialize)]
pub struct ApiConfig {
	owner: String,
	repo: String,
	github_token: String,
	runtime_configs: Vec<RuntimeConfig>,
}
impl ApiConfig {
	fn releases(&self) -> String {
		format!("https://api.github.com/repos/{}/{}/releases", self.owner, self.repo)
	}
}
/// Runtime information.
#[derive(Debug, Deserialize)]
pub struct RuntimeConfig {
	name: String,
	#[serde(deserialize_with = "deserialize_rpc_endpoint")]
	rpc_endpoints: Vec<Url>,
}

#[derive(Debug)]
struct Runtime {
	name: String,
	on_chain_version: u32,
	ok: bool,
}

#[derive(Debug, Deserialize)]
struct Release {
	assets: Vec<Asset>,
}
#[derive(Debug, Deserialize)]
struct Asset {
	browser_download_url: String,
}

#[derive(Debug, Deserialize)]
struct RpcResponse<R> {
	result: R,
}

fn deserialize_rpc_endpoint<'de, D>(d: D) -> Result<Vec<Url>, D::Error>
where
	D: Deserializer<'de>,
{
	<Vec<String>>::deserialize(d)?.iter().map(|s| s.parse().map_err(DeError::custom)).collect()
}

#[cfg(test)]
#[tokio::test]
async fn t() {
	tracing_subscriber::fmt::init();

	let api = Api::new(ApiConfig {
		owner: "polkadot-fellows".to_string(),
		repo: "runtimes".to_string(),
		github_token: env!("GITHUB_TOKEN").into(),
		runtime_configs: vec![
			RuntimeConfig {
				name: "kusama".into(),
				rpc_endpoints: vec!["https://kusama-rpc.polkadot.io".parse().unwrap()],
			},
			RuntimeConfig {
				name: "polkadot".into(),
				rpc_endpoints: vec!["https://rpc.polkadot.io".parse().unwrap()],
			},
		],
	});
}
