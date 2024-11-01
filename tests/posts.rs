#[cfg(test)]
mod posts {
	use dummy_json_rs::DummyJsonClient;

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
}
