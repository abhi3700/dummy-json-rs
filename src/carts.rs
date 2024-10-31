use crate::{DummyJsonClient, API_BASE_URL};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

const CARTS_BASE_URL: Lazy<String> = Lazy::new(|| format!("{}/carts", API_BASE_URL));

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CartProduct {
	pub id: u32,
	pub title: Option<String>,
	pub price: Option<f32>,
	pub quantity: Option<u32>,
	pub total: Option<f32>,
	#[serde(rename = "discountPercentage")]
	pub discount_percentage: Option<f32>,
	#[serde(rename = "discountedTotal")]
	pub discounted_total: Option<f32>,
	pub thumbnail: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Cart {
	pub id: u32,
	pub products: Vec<CartProduct>,
	pub total: f32,
	#[serde(rename = "discountedTotal")]
	pub discounted_total: f32,
	#[serde(rename = "userId")]
	pub user_id: u32,
	#[serde(rename = "totalProducts")]
	pub total_products: u32,
	#[serde(rename = "totalQuantity")]
	pub total_quantity: u32,
}

#[derive(Deserialize, Debug)]
pub struct GetAllCartsResponse {
	pub carts: Vec<Cart>,
	pub total: u32,
	pub skip: u32,
	pub limit: u32,
}

#[derive(Serialize, Debug)]
pub struct AddCartPayload {
	#[serde(rename = "userId")]
	pub user_id: u32,
	pub products: Vec<CartProduct>,
}

#[derive(Serialize, Debug, Default)]
pub struct UpdateCartPayload {
	#[serde(rename = "userId")]
	pub user_id: Option<u32>,
	pub merge: Option<bool>,
	pub products: Vec<CartProduct>,
}

#[derive(Deserialize, Debug)]
pub struct DeleteCartResponse {
	#[serde(flatten)]
	pub other_fields: Cart,
	#[serde(rename = "isDeleted")]
	pub is_deleted: bool,
	#[serde(rename = "deletedOn")]
	pub deleted_on: String,
}

impl DummyJsonClient {
	/// Get all carts
	pub async fn get_all_carts(&self) -> Result<GetAllCartsResponse, reqwest::Error> {
		let url = &*CARTS_BASE_URL;
		self.client.get(url).send().await?.json::<GetAllCartsResponse>().await
	}

	/// Get cart by id
	pub async fn get_cart_by_id(&self, id: u32) -> Result<Cart, reqwest::Error> {
		let url = format!("{}/{}", &*CARTS_BASE_URL, id);
		self.client.get(url).send().await?.json::<Cart>().await
	}

	/// Get carts of user
	pub async fn get_carts_of_user(
		&self,
		user_id: u32,
	) -> Result<GetAllCartsResponse, reqwest::Error> {
		let url = format!("{}/user/{}", &*CARTS_BASE_URL, user_id);
		self.client.get(url).send().await?.json::<GetAllCartsResponse>().await
	}

	/// Add cart
	pub async fn add_cart(&self, payload: AddCartPayload) -> Result<Cart, reqwest::Error> {
		let url = format!("{}/add", &*CARTS_BASE_URL);
		self.client.post(url).json(&payload).send().await?.json::<Cart>().await
	}

	/// Update cart
	pub async fn update_cart(
		&self,
		id: u32,
		payload: UpdateCartPayload,
	) -> Result<Cart, reqwest::Error> {
		let url = format!("{}/{}", &*CARTS_BASE_URL, id);
		self.client.put(url).json(&payload).send().await?.json::<Cart>().await
	}

	/// Delete cart
	pub async fn delete_cart(&self, cart_id: u32) -> Result<DeleteCartResponse, reqwest::Error> {
		let url = format!("{}/{}", &*CARTS_BASE_URL, cart_id);
		self.client.delete(url).send().await?.json::<DeleteCartResponse>().await
	}
}
