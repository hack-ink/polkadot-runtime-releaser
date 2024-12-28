// crates.io
use array_bytes::{Hex, TryFromHex};
use reqwew::{
	blocking::Http,
	reqwest::{blocking::Client, Method},
	Response,
};
use serde_json::Value;
// self
use super::*;

impl Wasmer {
	fn load_from_node() -> Self {
		let api = Client::default();
		let resp = api
			.request_with_retries(
				api.request(Method::POST, "https://polkadot.dotters.network")
					.json(&serde_json::json!(
						{
							"jsonrpc": "2.0",
							"id": 0,
							"method": "state_getStorage",
							"params": [b":code".hex("0x")]
						}
					))
					.build()
					.unwrap(),
				3,
				50,
			)
			.unwrap()
			.json::<Value>()
			.unwrap();
		let code = TryFromHex::try_from_hex(resp["result"].as_str().unwrap()).unwrap();
		let executor = WasmExecutor::default();
		let wasmer = Self { code, executor };

		wasmer
	}
}

#[test]
fn wasmer_should_work() {
	let wasmer = Wasmer::load_from_node();
	let maybe_ver = wasmer.runtime_version(true);

	assert!(maybe_ver.is_ok());

	let ver = maybe_ver.unwrap();

	assert_eq!(ver.spec_name, "polkadot");
	assert_eq!(ver.impl_name, "parity-polkadot");
	assert!(wasmer.metadata().is_ok());
}
