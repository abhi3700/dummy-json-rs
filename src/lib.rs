mod auth;
mod todos;

pub use auth::*;
use reqwest::Client;
pub use todos::*;

pub const API_BASE_URL: &str = "https://dummyjson.com";

pub struct DummyJsonClient {
	pub client: Client,
}

impl DummyJsonClient {
	pub fn new() -> Self {
		Self { client: Client::new() }
	}
}

impl Default for DummyJsonClient {
	fn default() -> Self {
		Self::new()
	}
}
