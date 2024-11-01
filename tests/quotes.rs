mod quotes {
	use dummy_json_rs::DummyJsonClient;

	#[tokio::test]
	async fn get_all_quotes() {
		let client = DummyJsonClient::default();
		let response = client.get_all_quotes().await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_quote_by_id() {
		let client = DummyJsonClient::default();
		let response = client.get_quote_by_id(1).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_random_quote() {
		let client = DummyJsonClient::default();
		let response = client.get_random_quote().await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_random_quotes() {
		let client = DummyJsonClient::default();
		let response = client.get_random_quotes(3).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn limit_skip_quotes() {
		let client = DummyJsonClient::default();
		let response = client.limit_skip_quotes(3, 10).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}
}
