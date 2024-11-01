mod comments {
	use dummy_json_rs::{AddComment, DummyJsonClient};

	#[tokio::test]
	async fn get_all_comments() {
		let client = DummyJsonClient::default();
		let response = client.get_all_comments().await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_comment_by_id() {
		let client = DummyJsonClient::default();
		let response = client.get_comment_by_id(1).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn limit_and_skip_comments() {
		let client = DummyJsonClient::default();
		let response = client.limit_and_skip_comments(10, 10, "body,postId").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_comments_by_post_id() {
		let client = DummyJsonClient::default();
		let response = client.get_comments_by_post_id(1).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn add_comment() {
		let client = DummyJsonClient::default();
		let comment =
			AddComment { body: "This makes all sense to me!".to_string(), post_id: 3, user_id: 5 };
		let response = client.add_comment(&comment).await;
		// assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn update_comment() {
		let client = DummyJsonClient::default();
		let response = client.update_comment(1, "This makes all sense to me! - updated").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn delete_comment() {
		let client = DummyJsonClient::default();
		let response = client.delete_comment(1).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}
}
