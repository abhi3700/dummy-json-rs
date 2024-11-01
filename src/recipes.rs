use crate::{DummyJsonClient, API_BASE_URL};
use once_cell::sync::Lazy;
use serde::Deserialize;

static RECIPES_BASE_URL: Lazy<String> = Lazy::new(|| format!("{}/recipes", API_BASE_URL));

#[derive(Deserialize, Debug)]
pub struct GetAllRecipesResponse {
	pub recipes: Vec<Recipe>,
	pub total: u32,
	pub skip: u32,
	pub limit: u32,
}

#[derive(Deserialize, Debug)]
pub struct Recipe {
	pub id: u32,
	pub name: Option<String>,
	pub ingredients: Option<Vec<String>>,
	pub instructions: Option<Vec<String>>,
	#[serde(rename = "prepTimeMinutes")]
	pub prep_time_mins: Option<u32>,
	#[serde(rename = "cookTimeMinutes")]
	pub cook_time_mins: Option<u32>,
	pub servings: Option<u32>,
	// TODO: convert the Easy, Difficulty to enum
	pub difficulty: Option<String>,
	pub cuisine: Option<String>,
	#[serde(rename = "caloriesPerServing")]
	pub calories_per_serving: Option<u32>,
	pub tags: Option<Vec<String>>,
	#[serde(rename = "userId")]
	pub user_id: Option<u32>,
	pub image: Option<String>,
	pub rating: Option<f32>,
	#[serde(rename = "reviewCount")]
	pub review_count: Option<u32>,
	#[serde(rename = "mealType")]
	pub meal_type: Option<Vec<String>>,
}

impl DummyJsonClient {
	/// Get all recipes
	pub async fn get_all_recipes(&self) -> Result<GetAllRecipesResponse, reqwest::Error> {
		self.client
			.get(RECIPES_BASE_URL.as_str())
			.send()
			.await?
			.json::<GetAllRecipesResponse>()
			.await
	}

	/// Get recipe by id
	pub async fn get_recipe_by_id(&self, id: u32) -> Result<Recipe, reqwest::Error> {
		self.client
			.get(format!("{}/{}", &*RECIPES_BASE_URL, id))
			.send()
			.await?
			.json::<Recipe>()
			.await
	}

	/// Search recipes
	pub async fn search_recipes(
		&self,
		query: &str,
	) -> Result<GetAllRecipesResponse, reqwest::Error> {
		self.client
			.get(format!("{}/search?q={}", &*RECIPES_BASE_URL, query))
			.send()
			.await?
			.json::<GetAllRecipesResponse>()
			.await
	}

	/// Limit and skip recipes
	pub async fn limit_and_skip_recipes(
		&self,
		limit: u32,
		skip: u32,
		selects: &str,
	) -> Result<GetAllRecipesResponse, reqwest::Error> {
		self.client
			.get(format!(
				"{}/?limit={}&skip={}&select={}",
				&*RECIPES_BASE_URL, limit, skip, selects
			))
			.send()
			.await?
			.json::<GetAllRecipesResponse>()
			.await
	}

	/// Sort recipes
	pub async fn sort_recipes(
		&self,
		sort_by: &str,
		// TODO: convert the asc, desc to enum
		order: &str,
	) -> Result<GetAllRecipesResponse, reqwest::Error> {
		self.client
			.get(format!("{}/?sortBy={}&order={}", &*RECIPES_BASE_URL, sort_by, order))
			.send()
			.await?
			.json::<GetAllRecipesResponse>()
			.await
	}

	/// Get recipes tags
	pub async fn get_recipes_tags(&self) -> Result<Vec<String>, reqwest::Error> {
		self.client
			.get(format!("{}/tags", &*RECIPES_BASE_URL))
			.send()
			.await?
			.json::<Vec<String>>()
			.await
	}

	/// Get recipes by tags
	pub async fn get_recipes_by_tags(
		&self,
		tags: &str,
	) -> Result<GetAllRecipesResponse, reqwest::Error> {
		self.client
			.get(format!("{}/tag/{}", &*RECIPES_BASE_URL, tags))
			.send()
			.await?
			.json::<GetAllRecipesResponse>()
			.await
	}

	/// Get recipes by meal type
	pub async fn get_recipes_by_meal_type(
		&self,
		meal_type: &str,
	) -> Result<GetAllRecipesResponse, reqwest::Error> {
		self.client
			.get(format!("{}/meal-type/{}", &*RECIPES_BASE_URL, meal_type))
			.send()
			.await?
			.json::<GetAllRecipesResponse>()
			.await
	}
}
