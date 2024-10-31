mod carts {
	use dummy_json_rs::{AddCartPayload, CartProduct, DummyJsonClient, UpdateCartPayload};

	#[tokio::test]
	async fn get_all_carts() {
		let client = DummyJsonClient::default();
		let response = client.get_all_carts().await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_cart_by_id() {
		let client = DummyJsonClient::default();
		let response = client.get_cart_by_id(1).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_carts_of_user() {
		let client = DummyJsonClient::default();
		let response = client.get_carts_of_user(33).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn add_cart() {
		let client = DummyJsonClient::default();
		let response = client
			.add_cart(AddCartPayload {
				user_id: 33,
				products: vec![
					CartProduct { id: 144, quantity: Some(4), ..Default::default() },
					CartProduct { id: 98, quantity: Some(1), ..Default::default() },
				],
			})
			.await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn update_cart() {
		let client = DummyJsonClient::default();
		let response = client
			.update_cart(
				1,
				UpdateCartPayload {
					user_id: Some(34),
					merge: Some(true),
					products: vec![
						CartProduct { id: 144, quantity: Some(4), ..Default::default() },
						CartProduct { id: 98, quantity: Some(1), ..Default::default() },
					],
				},
			)
			.await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn delete_cart() {
		let client = DummyJsonClient::default();
		let response = client.delete_cart(1).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}
}
