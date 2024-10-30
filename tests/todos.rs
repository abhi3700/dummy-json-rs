mod todos {
	use dummy_json_rs::DummyJsonClient;

	#[tokio::test]
	async fn get_all_todos() {
		let client = DummyJsonClient::default();
		let response = client.get_all_todos().await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_todo_by_id() {
		let client = DummyJsonClient::default();
		let response = client.get_todo_by_id(1).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_todo_by_id_fails_with_invalid_id() {
		let client = DummyJsonClient::default();
		let response = client.get_todo_by_id(24332).await;
		assert!(response.is_err());
	}

	#[tokio::test]
	async fn get_random_todo() {
		let client = DummyJsonClient::default();
		let response = client.get_random_todo().await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_random_todos() {
		let client = DummyJsonClient::default();
		let response = client.get_random_todos(3).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn limit_skip_todos() {
		let client = DummyJsonClient::default();
		let response = client.limit_skip_todos(3, 10).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn limit_skip_todos_get_all_todos() {
		let client = DummyJsonClient::default();
		let response = client.limit_skip_todos(0, 0).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_all_todos_of_user() {
		let client = DummyJsonClient::default();
		let response = client.get_all_todos_of_user(1).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn add_todo() {
		let client = DummyJsonClient::default();
		let response = client.add_todo("Use DummyJSON in the project", false, 5).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn update_todo() {
		let client = DummyJsonClient::default();
		let response = client.update_todo(1, true).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn delete_todo() {
		let client = DummyJsonClient::default();
		let response = client.delete_todo(1).await;
		// assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}
}
