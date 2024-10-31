mod auth;
mod todos;

pub use auth::*;
use reqwest::Client;
pub use todos::*;

pub const API_BASE_URL: &str = "https://dummyjson.com";

#[derive(Default)]
pub struct DummyJsonClient {
	pub client: Client,
}
