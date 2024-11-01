use crate::{DummyJsonClient, UserProfile, API_BASE_URL};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;

static COMMENTS_BASE_URL: Lazy<String> = Lazy::new(|| format!("{}/comments", API_BASE_URL));

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
	pub id: u32,
	pub body: String,
	#[serde(rename = "postId")]
	pub post_id: u32,
	pub user: UserProfile,
	pub likes: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddComment {
	pub body: String,
	#[serde(rename = "postId")]
	pub post_id: u32,
	#[serde(rename = "userId")]
	pub user_id: u32,
}

#[derive(Deserialize, Debug)]
pub struct GetAllComments {
	pub comments: Vec<Comment>,
	pub total: u32,
	pub skip: u32,
	pub limit: u32,
}

#[derive(Deserialize, Debug)]
pub struct DeleteCommentResponse {
	#[serde(flatten)]
	pub other_fields: Comment,
	#[serde(rename = "isDeleted")]
	pub is_deleted: bool,
	#[serde(rename = "deletedOn")]
	pub deleted_on: String,
}
impl DummyJsonClient {
	/// Get all comments
	pub async fn get_all_comments(&self) -> Result<GetAllComments, reqwest::Error> {
		self.client
			.get(COMMENTS_BASE_URL.as_str())
			.send()
			.await?
			.json::<GetAllComments>()
			.await
	}

	/// Get comment by id
	pub async fn get_comment_by_id(&self, id: u32) -> Result<Comment, reqwest::Error> {
		self.client
			.get(format!("{}/{}", COMMENTS_BASE_URL.as_str(), id))
			.send()
			.await?
			.json::<Comment>()
			.await
	}

	/// Limit and skip comments
	pub async fn limit_and_skip_comments(
		&self,
		limit: u32,
		skip: u32,
		select: &str,
	) -> Result<GetAllComments, reqwest::Error> {
		self.client
			.get(format!(
				"{}/?limit={}&skip={}&select={}",
				COMMENTS_BASE_URL.as_str(),
				limit,
				skip,
				select
			))
			.send()
			.await?
			.json::<GetAllComments>()
			.await
	}

	/// Get comments by post id
	pub async fn get_comments_by_post_id(
		&self,
		post_id: u32,
	) -> Result<GetAllComments, reqwest::Error> {
		self.client
			.get(format!("{}/?postId={}", COMMENTS_BASE_URL.as_str(), post_id))
			.send()
			.await?
			.json::<GetAllComments>()
			.await
	}

	/// Add comment
	pub async fn add_comment(&self, comment: &AddComment) -> Result<Comment, reqwest::Error> {
		self.client
			.post(format!("{}/add", COMMENTS_BASE_URL.as_str()))
			.json(comment)
			.send()
			.await?
			.json::<Comment>()
			.await
	}

	/// Update comment
	pub async fn update_comment(&self, id: u32, body: &str) -> Result<Comment, reqwest::Error> {
		let update_comment = &json!({ "body": body });
		self.client
			.put(format!("{}/{}", COMMENTS_BASE_URL.as_str(), id))
			.json(update_comment)
			.send()
			.await?
			.json::<Comment>()
			.await
	}

	/// Delete comment
	pub async fn delete_comment(&self, id: u32) -> Result<DeleteCommentResponse, reqwest::Error> {
		self.client
			.delete(format!("{}/{}", COMMENTS_BASE_URL.as_str(), id))
			.send()
			.await?
			.json::<DeleteCommentResponse>()
			.await
	}
}
