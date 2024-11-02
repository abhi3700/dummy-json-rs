use crate::{
	AddUserPayload, AllTodos, DummyJsonClient, GetAllCartsResponse, GetAllPosts, LoginRequest,
	LoginResponse, User, API_BASE_URL,
};
use once_cell::sync::Lazy;
use serde::Deserialize;

static USERS_BASE_URL: Lazy<String> = Lazy::new(|| format!("{}/users", API_BASE_URL));

#[derive(Deserialize, Debug)]
pub struct GetAllUsersResponse {
	pub users: Vec<User>,
	pub total: u32,
	pub skip: u32,
	pub limit: u32,
}

#[derive(Deserialize, Debug)]
pub struct DeleteUserResponse {
	#[serde(flatten)]
	pub user: User,
	#[serde(rename = "isDeleted")]
	pub is_deleted: bool,
	#[serde(rename = "deletedOn")]
	pub deleted_on: String,
}

impl DummyJsonClient {
	/// Get all users
	pub async fn get_all_users(&self) -> Result<GetAllUsersResponse, reqwest::Error> {
		self.client
			.get(USERS_BASE_URL.as_str())
			.send()
			.await?
			.json::<GetAllUsersResponse>()
			.await
	}

	/// Login User and get tokens
	pub async fn login_user(
		&self,
		username: &str,
		password: &str,
		expires_in_mins: Option<u32>,
	) -> Result<LoginResponse, reqwest::Error> {
		let payload: LoginRequest = LoginRequest {
			username: username.to_string(),
			password: password.to_string(),
			expires_in_mins,
		};
		self.client
			.post(format!("{}/login", USERS_BASE_URL.as_str()))
			.json(&payload)
			.send()
			.await?
			.json::<LoginResponse>()
			.await
	}

	/// Get current authenticated user
	pub async fn get_current_authenticated_user(
		&self,
		access_token: &str,
	) -> Result<User, reqwest::Error> {
		self.client
			.get(format!("{}/me", USERS_BASE_URL.as_str()))
			.header("Authorization", format!("Bearer {}", access_token))
			.send()
			.await?
			.json::<User>()
			.await
	}

	/// Get user by id
	pub async fn get_user_by_id(&self, id: u32) -> Result<User, reqwest::Error> {
		self.client
			.get(format!("{}/{}", USERS_BASE_URL.as_str(), id))
			.send()
			.await?
			.json::<User>()
			.await
	}

	/// Search users with name
	pub async fn search_users(&self, query: &str) -> Result<GetAllUsersResponse, reqwest::Error> {
		self.client
			.get(format!("{}/search?q={}", USERS_BASE_URL.as_str(), query))
			.send()
			.await?
			.json::<GetAllUsersResponse>()
			.await
	}

	/// Filter users by key and value
	pub async fn filter_users(
		&self,
		key: &str,
		value: &str,
	) -> Result<GetAllUsersResponse, reqwest::Error> {
		self.client
			.get(format!("{}/filter?key={}&value={}", USERS_BASE_URL.as_str(), key, value))
			.send()
			.await?
			.json::<GetAllUsersResponse>()
			.await
	}

	/// Limit, skip and select users
	pub async fn limit_skip_select_users(
		&self,
		limit: u32,
		skip: u32,
		selects: &str,
	) -> Result<GetAllUsersResponse, reqwest::Error> {
		self.client
			.get(format!(
				"{}/?limit={}&skip={}&selects={}",
				USERS_BASE_URL.as_str(),
				limit,
				skip,
				selects
			))
			.send()
			.await?
			.json::<GetAllUsersResponse>()
			.await
	}

	/// Sort users by key and order
	pub async fn sort_users(
		&self,
		key: &str,
		order: &str,
	) -> Result<GetAllUsersResponse, reqwest::Error> {
		self.client
			.get(format!("{}/?sortBy={}&order={}", USERS_BASE_URL.as_str(), key, order))
			.send()
			.await?
			.json::<GetAllUsersResponse>()
			.await
	}

	/// Get user carts by user id
	pub async fn get_user_carts_by_user_id(
		&self,
		user_id: u32,
	) -> Result<GetAllCartsResponse, reqwest::Error> {
		self.client
			.get(format!("{}/{}/carts", USERS_BASE_URL.as_str(), user_id))
			.send()
			.await?
			.json::<GetAllCartsResponse>()
			.await
	}

	/// Get user posts by user id
	pub async fn get_user_posts_by_user_id(
		&self,
		user_id: u32,
	) -> Result<GetAllPosts, reqwest::Error> {
		self.client
			.get(format!("{}/{}/posts", USERS_BASE_URL.as_str(), user_id))
			.send()
			.await?
			.json::<GetAllPosts>()
			.await
	}

	/// Get user todos by user id
	pub async fn get_user_todos_by_user_id(
		&self,
		user_id: u32,
	) -> Result<AllTodos, reqwest::Error> {
		self.client
			.get(format!("{}/{}/todos", USERS_BASE_URL.as_str(), user_id))
			.send()
			.await?
			.json::<AllTodos>()
			.await
	}

	/// Add user
	pub async fn add_user(&self, user: &AddUserPayload) -> Result<User, reqwest::Error> {
		self.client
			.post(format!("{}/add", USERS_BASE_URL.as_str()))
			.json(&user)
			.send()
			.await?
			.json::<User>()
			.await
	}

	/// Update user
	pub async fn update_user(
		&self,
		id: u32,
		user: &AddUserPayload,
	) -> Result<User, reqwest::Error> {
		self.client
			.put(format!("{}/{}", USERS_BASE_URL.as_str(), id))
			.json(&user)
			.send()
			.await?
			.json::<User>()
			.await
	}

	/// Delete user
	pub async fn delete_user(&self, id: u32) -> Result<DeleteUserResponse, reqwest::Error> {
		self.client
			.delete(format!("{}/{}", USERS_BASE_URL.as_str(), id))
			.send()
			.await?
			.json::<DeleteUserResponse>()
			.await
	}
}
