use crate::{DummyJsonClient, ProductCategory, API_BASE_URL};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static POSTS_BASE_URL: Lazy<String> = Lazy::new(|| format!("{}/posts", API_BASE_URL));

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Post {
	pub id: u32,
	#[serde(flatten)]
	pub other_fields: AddPost,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AddPost {
	pub title: Option<String>,
	pub body: Option<String>,
	pub tags: Option<Vec<String>>,
	pub reactions: Option<Reaction>,
	pub views: Option<u32>,
	#[serde(rename = "userId")]
	pub user_id: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct PostComment {
	pub id: u32,
	pub body: String,
	#[serde(rename = "postId")]
	pub post_id: u32,
	pub likes: u32,
	pub user: UserProfile,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile {
	pub id: u32,
	pub username: String,
	#[serde(rename = "fullName")]
	pub full_name: String,
}

#[derive(Deserialize, Debug)]
pub struct PostCommentsResponse {
	pub comments: Vec<PostComment>,
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

	/// Limit and skip posts
	pub async fn limit_and_skip_posts(
		&self,
		limit: u32,
		skip: u32,
		selects: &str,
	) -> Result<GetAllPosts, reqwest::Error> {
		self.client
			.get(format!(
				"{}/?limit={}&skip={}&select={}",
				POSTS_BASE_URL.as_str(),
				limit,
				skip,
				selects
			))
			.send()
			.await?
			.json::<GetAllPosts>()
			.await
	}

	/// Sort posts
	pub async fn sort_posts(
		&self,
		sort_by: &str,
		// TODO: Change to enum
		order: &str,
	) -> Result<GetAllPosts, reqwest::Error> {
		self.client
			.get(format!("{}/?sortBy={}&order={}", POSTS_BASE_URL.as_str(), sort_by, order))
			.send()
			.await?
			.json::<GetAllPosts>()
			.await
	}

	/// Get all posts tags
	pub async fn get_all_posts_tags(&self) -> Result<Vec<ProductCategory>, reqwest::Error> {
		self.client
			.get(format!("{}/tags", POSTS_BASE_URL.as_str()))
			.send()
			.await?
			.json::<Vec<ProductCategory>>()
			.await
	}

	/// Get posts by tags
	pub async fn get_posts_by_tags(&self, tag: &str) -> Result<GetAllPosts, reqwest::Error> {
		self.client
			.get(format!("{}/tag/{}", POSTS_BASE_URL.as_str(), tag))
			.send()
			.await?
			.json::<GetAllPosts>()
			.await
	}

	/// Get posts by user id
	pub async fn get_posts_by_user_id(&self, user_id: u32) -> Result<GetAllPosts, reqwest::Error> {
		self.client
			.get(format!("{}/user/{}", POSTS_BASE_URL.as_str(), user_id))
			.send()
			.await?
			.json::<GetAllPosts>()
			.await
	}

	/// Get post comments
	pub async fn get_post_comments(&self, id: u32) -> Result<PostCommentsResponse, reqwest::Error> {
		self.client
			.get(format!("{}/{}/comments", POSTS_BASE_URL.as_str(), id))
			.send()
			.await?
			.json::<PostCommentsResponse>()
			.await
	}

	/// Add post
	pub async fn add_post(&self, post: &AddPost) -> Result<Post, reqwest::Error> {
		self.client
			.post(format!("{}/add", POSTS_BASE_URL.as_str()))
			.json(post)
			.send()
			.await?
			.json::<Post>()
			.await
	}

	/// Update post
	pub async fn update_post(&self, id: u32, post: &AddPost) -> Result<AddPost, reqwest::Error> {
		self.client
			.put(format!("{}/{}", POSTS_BASE_URL.as_str(), id))
			.json(post)
			.send()
			.await?
			.json::<AddPost>()
			.await
	}

	/// Delete post
	pub async fn delete_post(&self, id: u32) -> Result<Post, reqwest::Error> {
		self.client
			.delete(format!("{}/{}", POSTS_BASE_URL.as_str(), id))
			.send()
			.await?
			.json::<Post>()
			.await
	}
}
