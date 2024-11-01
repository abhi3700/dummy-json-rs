mod auth;
mod carts;
mod products;
mod recipes;
mod todos;

pub use auth::*;
pub use carts::*;
pub use products::*;
pub use recipes::*;
use reqwest::Client;
pub use todos::*;

const API_BASE_URL: &str = "https://dummyjson.com";

#[derive(Default)]
pub struct DummyJsonClient {
	pub client: Client,
}
