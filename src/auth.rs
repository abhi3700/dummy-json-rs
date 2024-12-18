//! Auth module
//! The auth endpoint provides details about the user authentication and authorization and refresh
//! tokens.
//!
//! https://dummyjson.com/docs/auth

use crate::{DummyJsonClient, API_BASE_URL};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;

static AUTH_BASE_URL: Lazy<String> = Lazy::new(|| format!("{}/auth", API_BASE_URL));

/// Login request payload
#[derive(Serialize)]
pub struct LoginRequest {
	pub username: String,
	pub password: String,
	#[serde(rename = "expiresInMins")]
	pub expires_in_mins: Option<u32>,
}

/// Login response
#[derive(Deserialize, Debug)]
pub struct LoginResponse {
	pub id: u32,
	pub username: String,
	pub email: String,
	#[serde(rename = "firstName")]
	pub first_name: String,
	#[serde(rename = "lastName")]
	pub last_name: String,
	pub gender: String,
	pub image: String,
	/// JWT accessToken (for backward compatibility) in response and cookies
	#[serde(rename = "accessToken")]
	pub access_token: String,
	/// refreshToken in response and cookies
	#[serde(rename = "refreshToken")]
	pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
	pub id: u32,
	#[serde(flatten)]
	pub other_fields: AddUserPayload,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AddUserPayload {
	#[serde(rename = "firstName")]
	pub first_name: Option<String>,
	#[serde(rename = "lastName")]
	pub last_name: Option<String>,
	#[serde(rename = "maidenName")]
	pub maiden_name: Option<String>,
	pub age: Option<u8>,
	pub gender: Option<String>,
	pub email: Option<String>,
	pub phone: Option<String>,
	pub username: Option<String>,
	pub password: Option<String>,
	#[serde(rename = "birthDate")]
	pub birth_date: Option<String>,
	pub image: Option<String>,
	#[serde(rename = "bloodGroup")]
	pub blood_group: Option<String>,
	pub height: Option<f32>,
	pub weight: Option<f32>,
	#[serde(rename = "eyeColor")]
	pub eye_color: Option<String>,
	pub hair: Option<Hair>,
	// TODO: Add other fields
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Hair {
	pub color: String,
	#[serde(rename = "type")]
	pub r#type: String,
}

/// Refresh response
#[derive(Deserialize, Debug)]
pub struct RefreshResponse {
	#[serde(rename = "accessToken")]
	pub access_token: String,
	#[serde(rename = "refreshToken")]
	pub refresh_token: String,
}

impl DummyJsonClient {
	/// Login to the dummyjson API
	pub async fn login(
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

		let url = format!("{}/login", AUTH_BASE_URL.as_str());
		let response = self.client.post(url).json(&payload).send().await?;
		response.json().await
	}

	/// Get the current user
	pub async fn get_user(&self, access_token: &str) -> Result<User, reqwest::Error> {
		let url = format!("{}/me", AUTH_BASE_URL.as_str());
		let response = self
			.client
			.get(url)
			.header("Authorization", format!("Bearer {}", access_token))
			.send()
			.await?;
		response.json().await
	}

	/// Refresh the auth session
	pub async fn refresh_auth_session(
		&self,
		refresh_token: &str,
		expires_in_mins: u32,
	) -> Result<RefreshResponse, reqwest::Error> {
		let payload = json!({
			"refreshToken": refresh_token,
			"expiresInMins": expires_in_mins,
		});

		let url = format!("{}/refresh", AUTH_BASE_URL.as_str());
		let response = self.client.post(url).json(&payload).send().await?;
		response.json().await
	}
}
