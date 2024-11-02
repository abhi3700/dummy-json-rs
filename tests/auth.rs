#[cfg(test)]
mod auth {
	use dummy_json_rs::DummyJsonClient;

	#[tokio::test]
	async fn login() {
		let client = DummyJsonClient::default();
		let response = client.login("emilys", "emilyspass", Some(30)).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn get_user() {
		let client = DummyJsonClient::default();
		let response = client.login("emilys", "emilyspass", Some(30)).await;
		let access_token = response.unwrap().access_token;
		let response = client.get_user(&access_token).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}

	#[tokio::test]
	async fn refresh_auth_session() {
		let client = DummyJsonClient::default();
		let response = client.login("emilys", "emilyspass", Some(30)).await;
		let refresh_token = response.unwrap().refresh_token;
		let response = client.refresh_auth_session(&refresh_token, 30).await;
		assert!(response.is_ok());
		println!("{:#?}", response.unwrap());
	}
}
