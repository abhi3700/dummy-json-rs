use crate::{DummyJsonClient, API_BASE_URL};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static QUOTES_BASE_URL: Lazy<String> = Lazy::new(|| format!("{}/quotes", API_BASE_URL));

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
	pub id: u32,
	pub quote: String,
	pub author: String,
}

#[derive(Deserialize, Debug)]
pub struct GetAllQuotes {
	pub quotes: Vec<Quote>,
	pub total: u32,
	pub skip: u32,
	pub limit: u32,
}

impl DummyJsonClient {
	/// Get all quotes
	pub async fn get_all_quotes(&self) -> Result<GetAllQuotes, reqwest::Error> {
		self.client
			.get(QUOTES_BASE_URL.as_str())
			.send()
			.await?
			.json::<GetAllQuotes>()
			.await
	}

	/// Get quote by id
	pub async fn get_quote_by_id(&self, id: u32) -> Result<Quote, reqwest::Error> {
		self.client
			.get(format!("{}/{}", QUOTES_BASE_URL.as_str(), id))
			.send()
			.await?
			.json::<Quote>()
			.await
	}

	/// Get random quote
	pub async fn get_random_quote(&self) -> Result<Quote, reqwest::Error> {
		self.client
			.get(format!("{}/random", QUOTES_BASE_URL.as_str()))
			.send()
			.await?
			.json::<Quote>()
			.await
	}

	/// Get random quotes
	pub async fn get_random_quotes(&self, count: u32) -> Result<Vec<Quote>, reqwest::Error> {
		self.client
			.get(format!("{}/random/{}", QUOTES_BASE_URL.as_str(), count))
			.send()
			.await?
			.json::<Vec<Quote>>()
			.await
	}

	/// Limit and skip quotes
	pub async fn limit_skip_quotes(
		&self,
		limit: u32,
		skip: u32,
	) -> Result<GetAllQuotes, reqwest::Error> {
		self.client
			.get(format!("{}/?limit={}&skip={}", QUOTES_BASE_URL.as_str(), limit, skip))
			.send()
			.await?
			.json::<GetAllQuotes>()
			.await
	}
}
