//! Todos module
//! https://dummyjson.com/docs/todos

use crate::{DummyJsonClient, API_BASE_URL};
use once_cell::sync::Lazy;
use reqwest::Error;
use serde::Deserialize;
use serde_json::json;

const TODOS_BASE_URL: Lazy<String> = Lazy::new(|| format!("{}/todos", API_BASE_URL));

/// Todo item
#[derive(Deserialize, Debug)]
pub struct Todo {
	pub id: u32,
	pub todo: String,
	pub completed: bool,
	#[serde(rename = "userId")]
	pub user_id: u32,
}

/// All Todos response
#[derive(Deserialize, Debug)]
pub struct AllTodos {
	pub todos: Vec<Todo>,
	pub total: u32,
	pub skip: u32,
	pub limit: u32,
}

/// Delete todo response
#[derive(Deserialize, Debug)]
pub struct DeleteTodoResponse {
	#[serde(flatten)]
	pub todo: Todo,
	#[serde(rename = "isDeleted")]
	pub is_deleted: bool,
	#[serde(rename = "deletedOn")]
	pub deleted_on: String,
}

impl DummyJsonClient {
	/// Get all todos
	pub async fn get_all_todos(&self) -> Result<AllTodos, Error> {
		let url = &*TODOS_BASE_URL;
		self.client.get(url).send().await?.json().await
	}

	/// Get todo by id
	pub async fn get_todo_by_id(&self, id: u32) -> Result<Todo, Error> {
		let url = format!("{}/{}", *TODOS_BASE_URL, id);
		self.client.get(url).send().await?.json().await
	}

	/// Get random todo
	pub async fn get_random_todo(&self) -> Result<Todo, Error> {
		let url = format!("{}/random", *TODOS_BASE_URL);
		self.client.get(url).send().await?.json().await
	}

	/// Get random todos
	pub async fn get_random_todos(&self, count: u32) -> Result<Vec<Todo>, Error> {
		let url = format!("{}/random/{}", *TODOS_BASE_URL, count);
		self.client.get(url).send().await?.json().await
	}

	/// Limit and skip todos
	pub async fn limit_skip_todos(&self, limit: u32, skip: u32) -> Result<AllTodos, Error> {
		let url = format!("{}/?limit={}&skip={}", *TODOS_BASE_URL, limit, skip);
		self.client.get(url).send().await?.json().await
	}

	/// Get all todos of user
	pub async fn get_all_todos_of_user(&self, user_id: u32) -> Result<AllTodos, Error> {
		let url = format!("{}/user/{}", *TODOS_BASE_URL, user_id);
		self.client.get(url).send().await?.json().await
	}

	/// Add todo
	pub async fn add_todo(&self, todo: &str, completed: bool, user_id: u32) -> Result<Todo, Error> {
		let url = format!("{}/add", *TODOS_BASE_URL);
		let payload = json!({ "todo": todo, "completed": completed, "userId": user_id });
		self.client.post(url).json(&payload).send().await?.json().await
	}

	/// Update todo
	pub async fn update_todo(&self, id: u32, completed: bool) -> Result<Todo, Error> {
		let url = format!("{}/{}", *TODOS_BASE_URL, id);
		let payload = json!({ "completed": completed });
		self.client.put(url).json(&payload).send().await?.json().await
	}

	/// Delete todo
	pub async fn delete_todo(&self, id: u32) -> Result<DeleteTodoResponse, Error> {
		let url = format!("{}/{}", *TODOS_BASE_URL, id);
		self.client.delete(url).send().await?.json().await
	}
}
