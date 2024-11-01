#[cfg(test)]
mod posts {
	use dummy_json_rs::{AddPost, DummyJsonClient};

	#[tokio::test]
	async fn get_all_posts() {
		let client = DummyJsonClient::default();
		let response = client.get_all_posts().await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_post_by_id() {
		let client = DummyJsonClient::default();
		let response = client.get_post_by_id(1).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn search_posts() {
		let client = DummyJsonClient::default();
		let response = client.search_posts("love").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn limit_and_skip_posts() {
		let client = DummyJsonClient::default();
		let response = client.limit_and_skip_posts(10, 10, "title,reactions,userId").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn sort_posts() {
		let client = DummyJsonClient::default();
		let response = client.sort_posts("title", "desc").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_all_posts_tags() {
		let client = DummyJsonClient::default();
		let response = client.get_all_posts_tags().await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_posts_by_tags() {
		let client = DummyJsonClient::default();
		let response = client.get_posts_by_tags("nature").await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_posts_by_user_id() {
		let client = DummyJsonClient::default();
		let response = client.get_posts_by_user_id(1).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_post_comments() {
		let client = DummyJsonClient::default();
		let response = client.get_post_comments(1).await;
		// assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn add_post() {
		let client = DummyJsonClient::default();
		let post = AddPost {
			title: Some("test title".to_string()),
			body: Some("test body".to_string()),
			user_id: Some(1),
			tags: Some(vec!["nature".to_string(), "photography".to_string()]),
			..Default::default()
		};
		let response = client.add_post(&post).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn update_post() {
		let client = DummyJsonClient::default();
		let updated_post = AddPost {
			title: Some("updated title".to_string()),
			body: Some("updated body".to_string()),
			..Default::default()
		};
		let response = client.update_post(1, &updated_post).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn delete_post() {
		let client = DummyJsonClient::default();
		let response = client.delete_post(1).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}
}
