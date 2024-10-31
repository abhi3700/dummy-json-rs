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
	/// Expiration time in minutes
	#[serde(rename = "expiresInMins")]
	pub expires_in_mins: u32,
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

#[derive(Deserialize, Debug)]
pub struct User {
	pub id: u32,
	#[serde(rename = "firstName")]
	pub first_name: String,
	#[serde(rename = "lastName")]
	pub last_name: String,
	#[serde(rename = "maidenName")]
	pub maiden_name: String,
	pub age: u8,
	pub gender: String,
	pub email: String,
	pub phone: String,
	pub username: String,
	pub password: String,
	#[serde(rename = "birthDate")]
	pub birth_date: String,
	pub image: String,
	#[serde(rename = "bloodGroup")]
	pub blood_group: String,
	pub height: f32,
	pub weight: f32,
	#[serde(rename = "eyeColor")]
	pub eye_color: String,
	pub hair: Hair,
	// TODO: Other fields
}

#[derive(Deserialize, Debug)]
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
		expires_in_mins: u32,
	) -> Result<LoginResponse, reqwest::Error> {
		let payload: LoginRequest = LoginRequest {
			username: username.to_string(),
			password: password.to_string(),
			expires_in_mins,
		};

		let url = format!("{}/login", *AUTH_BASE_URL);
		let response = self.client.post(url).json(&payload).send().await?;
		response.json().await
	}

	/// Get the current user
	pub async fn get_user(&self, access_token: &str) -> Result<User, reqwest::Error> {
		let url = format!("{}/me", *AUTH_BASE_URL);
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

		let url = format!("{}/refresh", *AUTH_BASE_URL);
		let response = self.client.post(url).json(&payload).send().await?;
		response.json().await
	}
}
