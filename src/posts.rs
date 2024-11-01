use crate::{DummyJsonClient, API_BASE_URL};
use once_cell::sync::Lazy;
use serde::Deserialize;

static POSTS_BASE_URL: Lazy<String> = Lazy::new(|| format!("{}/posts", API_BASE_URL));

#[derive(Deserialize, Debug)]
pub struct Post {
	pub id: u32,
	pub title: String,
	pub body: String,
	pub tags: Vec<String>,
	pub reactions: Reaction,
	pub views: u32,
	#[serde(rename = "userId")]
	pub user_id: u32,
}

#[derive(Deserialize, Debug)]
pub struct Reaction {
	pub likes: u32,
	pub dislikes: u32,
}

#[derive(Deserialize, Debug)]
pub struct GetAllPosts {
	pub posts: Vec<Post>,
	pub total: u32,
	pub skip: u32,
	pub limit: u32,
}

impl DummyJsonClient {
	/// Get all posts
	pub async fn get_all_posts(&self) -> Result<GetAllPosts, reqwest::Error> {
		self.client
			.get(POSTS_BASE_URL.as_str())
			.send()
			.await?
			.json::<GetAllPosts>()
			.await
	}

	/// Get post by id
	pub async fn get_post_by_id(&self, id: u32) -> Result<Post, reqwest::Error> {
		self.client
			.get(format!("{}/{}", POSTS_BASE_URL.as_str(), id))
			.send()
			.await?
			.json::<Post>()
			.await
	}

	/// Search posts
	pub async fn search_posts(&self, query: &str) -> Result<GetAllPosts, reqwest::Error> {
		self.client
			.get(format!("{}/search?q={}", POSTS_BASE_URL.as_str(), query))
			.send()
			.await?
			.json::<GetAllPosts>()
			.await
	}
}
