#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use std::collections::HashMap;
use sass_layer::Consumer;

use sass_layer::consumer::{consumer_list::ConsumerList};

#[derive(Debug)]
struct ApiKey<'r>(&'r str);

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
}

#[get("/")]
fn index(_key: ApiKey) -> &'static str {
    "Hello, world!"
}

use rocket::tokio::time::{sleep, Duration};

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[launch]
fn rocket() -> _ {
    let consumers = vec![Consumer::new(&HashMap::from([
        ("id", "1"),
        ("access_token", "user-1"),
    ]))];

    rocket::build()
        .mount("/", routes![index, delay])
        .manage(ConsumerList { consumers })
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        async fn is_valid(req: &Request<'_>, key: &str) -> bool {
            let consumer_list = req.guard::<&State<ConsumerList>>().await.unwrap();
            match consumer_list.get_by_access_token(key) {
                Some(_) => true,
                None => false,
            }
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing)),
            Some(key) if is_valid(req, key).await => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::{Header, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .get(uri!(super::index))
            .header(Header {
                name: "x-api-key".into(),
                value: "user-1".into(),
            })
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }

    #[test]
    fn auth_check() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::index)).dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }
}
