mod auth;
mod carts;
mod comments;
mod posts;
mod products;
mod quotes;
mod recipes;
mod todos;

pub use auth::*;
pub use carts::*;
pub use comments::*;
pub use posts::*;
pub use products::*;
pub use quotes::*;
pub use recipes::*;
use reqwest::Client;
pub use todos::*;

const API_BASE_URL: &str = "https://dummyjson.com";

#[derive(Default)]
pub struct DummyJsonClient {
	pub client: Client,
}
