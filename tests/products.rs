#[cfg(test)]
mod products {
	use dummy_json_rs::{AddProduct, DummyJsonClient};

	#[tokio::test]
	async fn get_all_products() {
		let client = DummyJsonClient::default();
		let response = client.get_all_products().await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_product_by_id() {
		let client = DummyJsonClient::default();
		let response = client.get_product_by_id(1).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn search_products() {
		let client = DummyJsonClient::default();
		let response = client.search_products("phone").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn limit_and_skip_products() {
		let client = DummyJsonClient::default();
		let response = client.limit_and_skip_products(1, 10, "title,price").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn sort_products() {
		let client = DummyJsonClient::default();
		let response = client.sort_products_by("title", "asc").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_product_categories() {
		let client = DummyJsonClient::default();
		let response = client.get_product_categories().await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_product_categories_list() {
		let client = DummyJsonClient::default();
		let response = client.get_product_categories_list().await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_products_by_category() {
		let client = DummyJsonClient::default();
		let response = client.get_products_by_category("smartphones").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn add_product() {
		let client = DummyJsonClient::default();
		let response = client
			.add_product(&AddProduct {
				title: "iPhone 18".to_string(),
				description: Some("New phone released in 2024".to_string()),
				price: Some(1200.0),
				..Default::default()
			})
			.await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn update_product() {
		let client = DummyJsonClient::default();
		let response = client
			.update_product(1, &AddProduct { title: "iPhone 19".to_string(), ..Default::default() })
			.await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn delete_product() {
		let client = DummyJsonClient::default();
		let response = client.delete_product(1).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}
}
