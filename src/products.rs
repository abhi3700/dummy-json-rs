use crate::{DummyJsonClient, API_BASE_URL};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static PRODUCTS_BASE_URL: Lazy<String> = Lazy::new(|| format!("{}/products", API_BASE_URL));

#[derive(Deserialize, Debug)]
pub struct Product {
	pub id: u32,
	#[serde(flatten)]
	pub other_fields: AddProduct,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AddProduct {
	pub title: String,
	pub description: Option<String>,
	pub price: Option<f32>,
	#[serde(rename = "discountPercentage")]
	pub discount_percentage: Option<f32>,
	pub rating: Option<f32>,
	pub stock: Option<u32>,
	pub tags: Option<Vec<String>>,
	// FIXME: Not sure, why the actual API response missing 'brand' field
	// resulting in error.
	// pub brand: Option<String>,
	pub sku: Option<String>,
	pub weight: Option<u16>,
	pub dimensions: Option<Dimension>,
	#[serde(rename = "warrantyInformation")]
	pub warranty_info: Option<String>,
	#[serde(rename = "shippingInformation")]
	pub shipping_info: Option<String>,
	#[serde(rename = "availabilityStatus")]
	pub availability_status: Option<String>,
	pub reviews: Option<Vec<Review>>,
	#[serde(rename = "returnPolicy")]
	pub return_policy: Option<String>,
	#[serde(rename = "minimumOrderQuantity")]
	pub min_order_qty: Option<u16>,
	pub meta: Option<Meta>,
	pub images: Option<Vec<String>>,
	pub thumbnail: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dimension {
	pub width: f32,
	pub height: f32,
	pub depth: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Review {
	pub rating: u8,
	pub comment: String,
	pub date: String,
	#[serde(rename = "reviewerName")]
	pub reviewer_name: String,
	#[serde(rename = "reviewerEmail")]
	pub reviewer_email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
	#[serde(rename = "createdAt")]
	pub created_at: String,
	#[serde(rename = "updatedAt")]
	pub updated_at: String,
	pub barcode: String,
	#[serde(rename = "qrCode")]
	pub qr_code: String,
}

#[derive(Deserialize, Debug)]
pub struct GetAllProductsResponse {
	pub products: Vec<Product>,
	pub total: u32,
	pub skip: u32,
	pub limit: u32,
}

#[derive(Deserialize, Debug)]
pub struct ProductCategory {
	pub slug: String,
	pub name: String,
	pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct DeleteProductResponse {
	#[serde(flatten)]
	pub other_fields: Product,
	#[serde(rename = "isDeleted")]
	pub is_deleted: bool,
	#[serde(rename = "deletedOn")]
	pub deleted_on: String,
}

impl DummyJsonClient {
	/// Get all products
	pub async fn get_all_products(&self) -> Result<GetAllProductsResponse, reqwest::Error> {
		let response = self.client.get(PRODUCTS_BASE_URL.as_str()).send().await?;
		response.json::<GetAllProductsResponse>().await
	}

	/// Get product by id
	pub async fn get_product_by_id(&self, id: u32) -> Result<Product, reqwest::Error> {
		let url = &format!("{}/{}", PRODUCTS_BASE_URL.as_str(), id);
		let response = self.client.get(url).send().await?;
		response.json::<Product>().await
	}

	/// Search products
	pub async fn search_products(
		&self,
		query: &str,
	) -> Result<GetAllProductsResponse, reqwest::Error> {
		let url = &format!("{}/search?q={}", PRODUCTS_BASE_URL.as_str(), query);
		let response = self.client.get(url).send().await?;
		response.json::<GetAllProductsResponse>().await
	}

	/// Limit and skip products
	pub async fn limit_and_skip_products(
		&self,
		limit: u32,
		skip: u32,
		selects: &str,
	) -> Result<GetAllProductsResponse, reqwest::Error> {
		let url = &format!(
			"{}/?limit={}&skip={}&select={}",
			PRODUCTS_BASE_URL.as_str(),
			limit,
			skip,
			selects
		);
		let response = self.client.get(url).send().await?;
		response.json::<GetAllProductsResponse>().await
	}

	/// Sort products by field
	pub async fn sort_products_by(
		&self,
		field: &str,
		order: &str,
	) -> Result<GetAllProductsResponse, reqwest::Error> {
		let url = &format!("{}/?sortBy={}&order={}", PRODUCTS_BASE_URL.as_str(), field, order);
		let response = self.client.get(url).send().await?;
		response.json::<GetAllProductsResponse>().await
	}

	/// Get product categories
	pub async fn get_product_categories(&self) -> Result<Vec<ProductCategory>, reqwest::Error> {
		let url = &format!("{}/categories", PRODUCTS_BASE_URL.as_str());
		let response = self.client.get(url).send().await?;
		response.json::<Vec<ProductCategory>>().await
	}

	/// Get product categories list
	pub async fn get_product_categories_list(&self) -> Result<Vec<String>, reqwest::Error> {
		let url = &format!("{}/category-list", PRODUCTS_BASE_URL.as_str());
		let response = self.client.get(url).send().await?;
		response.json::<Vec<String>>().await
	}

	/// Get products by category
	pub async fn get_products_by_category(
		&self,
		category: &str,
	) -> Result<GetAllProductsResponse, reqwest::Error> {
		let url = &format!("{}/category/{}", PRODUCTS_BASE_URL.as_str(), category);
		let response = self.client.get(url).send().await?;
		response.json::<GetAllProductsResponse>().await
	}

	/// Add product
	pub async fn add_product(&self, product: &AddProduct) -> Result<Product, reqwest::Error> {
		let url = &format!("{}/add", PRODUCTS_BASE_URL.as_str());
		let response = self.client.post(url).json(product).send().await?;
		response.json::<Product>().await
	}

	/// Update product
	pub async fn update_product(
		&self,
		id: u32,
		product: &AddProduct,
	) -> Result<Product, reqwest::Error> {
		let url = &format!("{}/{}", PRODUCTS_BASE_URL.as_str(), id);
		let response = self.client.put(url).json(product).send().await?;
		response.json::<Product>().await
	}

	/// Delete product
	pub async fn delete_product(&self, id: u32) -> Result<DeleteProductResponse, reqwest::Error> {
		let url = &format!("{}/{}", PRODUCTS_BASE_URL.as_str(), id);
		let response = self.client.delete(url).send().await?;
		response.json::<DeleteProductResponse>().await
	}
}
