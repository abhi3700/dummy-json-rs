mod auth;
mod products;
mod todos;

pub use auth::*;
pub use products::*;
use reqwest::Client;
pub use todos::*;

const API_BASE_URL: &str = "https://dummyjson.com";

#[derive(Default)]
pub struct DummyJsonClient {
	pub client: Client,
}
