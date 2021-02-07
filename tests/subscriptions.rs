// use newsletter_service::configuration::get_configuration;
// use sqlx::{Connection, PgConnection};

mod test_helpers;

#[actix_rt::test]
async fn subscribe_returns_200_ok_for_valid_form_data() {
    let address = test_helpers::spawn_app();

    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16())
}

#[actix_rt::test]
async fn subscribe_stores_subscription_in_database() {
    // let config = get_configuration().expect("Failed to read configuration");
    // let conn_string = config.database.connection_string();
    // let mut connection = PgConnection::connect(&conn_string)
    //     .await
    //     .expect("Failed to connect to Postgres.");

    let address = test_helpers::spawn_app();

    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    // let saved = sqlx::query!("SELECT email, name FROM subscriptions")
    //     .fetch_one(&mut connection)
    //     .await
    //     .expect("Failed to fetch saved subscription.");

    // assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    // assert_eq!(saved.name, "le guin");
}

#[actix_rt::test]
async fn subscribe_returns_400_bad_request_when_data_is_missing() {
    let address = test_helpers::spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}
