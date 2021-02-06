mod test_helpers;

#[actix_rt::test]
async fn health_check_works() {
    let address = test_helpers::spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(0, response.content_length().unwrap());
}
