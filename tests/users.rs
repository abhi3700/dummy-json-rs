mod users {
	use dummy_json_rs::{AddUserPayload, DummyJsonClient};

	#[tokio::test]
	async fn get_all_users() {
		let client = DummyJsonClient::default();
		let response = client.get_all_users().await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn login_user() {
		let client = DummyJsonClient::default();
		let response = client.login_user("emilys", "emilyspass", None).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn login_user_with_expires_in_mins() {
		let client = DummyJsonClient::default();
		let response = client.login_user("emilys", "emilyspass", Some(30)).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_current_authenticated_user() {
		let client = DummyJsonClient::default();
		let response = client.login_user("emilys", "emilyspass", None).await;
		let access_token = response.unwrap().access_token;
		let response = client.get_current_authenticated_user(&access_token).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_user_by_id() {
		let client = DummyJsonClient::default();
		let response = client.get_user_by_id(1).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn search_users() {
		let client = DummyJsonClient::default();
		let response = client.search_users("john").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn filter_users() {
		let client = DummyJsonClient::default();
		let response = client.filter_users("hair.color", "Brown").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn limit_skip_select_users() {
		let client = DummyJsonClient::default();
		let response = client.limit_skip_select_users(3, 10, "firstName,age").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn sort_users() {
		let client = DummyJsonClient::default();
		let response = client.sort_users("age", "desc").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_user_carts_by_user_id() {
		let client = DummyJsonClient::default();
		let response = client.get_user_carts_by_user_id(6).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_user_posts_by_user_id() {
		let client = DummyJsonClient::default();
		let response = client.get_user_posts_by_user_id(6).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_user_todos_by_user_id() {
		let client = DummyJsonClient::default();
		let response = client.get_user_todos_by_user_id(6).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn add_user() {
		let client = DummyJsonClient::default();
		let payload = AddUserPayload {
			first_name: Some("John".to_string()),
			last_name: Some("Doe".to_string()),
			age: Some(25),
			..Default::default()
		};
		let response = client.add_user(&payload).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn update_user() {
		let client = DummyJsonClient::default();
		let payload = AddUserPayload {
			last_name: Some("Owais".to_string()),
			age: Some(27),
			..Default::default()
		};
		let response = client.update_user(6, &payload).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn delete_user() {
		let client = DummyJsonClient::default();
		let response = client.delete_user(6).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}
}
