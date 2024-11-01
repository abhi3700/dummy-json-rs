mod recipes {
	use dummy_json_rs::DummyJsonClient;

	#[tokio::test]
	async fn get_all_recipes() {
		let client = DummyJsonClient::default();
		let response = client.get_all_recipes().await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_recipe_by_id() {
		let client = DummyJsonClient::default();
		let response = client.get_recipe_by_id(1).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn search_recipes() {
		let client = DummyJsonClient::default();
		let response = client.search_recipes("pizza").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn limit_and_skip_recipes() {
		let client = DummyJsonClient::default();
		let response = client.limit_and_skip_recipes(10, 10, "name,image").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn sort_recipes() {
		let client = DummyJsonClient::default();
		let response = client.sort_recipes("name", "asc").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_recipes_tags() {
		let client = DummyJsonClient::default();
		let response = client.get_recipes_tags().await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_recipes_by_tags() {
		let client = DummyJsonClient::default();
		let response = client.get_recipes_by_tags("biryani").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_recipes_by_meal_type() {
		let client = DummyJsonClient::default();
		let response = client.get_recipes_by_meal_type("snack").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}
}
