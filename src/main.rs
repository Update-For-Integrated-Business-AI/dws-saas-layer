#[macro_use]
extern crate rocket;

use std::sync::Mutex;

use uws_gateway::consumer::consumer_list::ConsumerList;

use uws_gateway::db::file_db::FlatTable;
use uws_gateway::guards::{ApiKey, HostHeader};

#[get("/")]
fn index(key: ApiKey, _host: HostHeader) -> String {
    println!("New request from {:?}", key);
    "Hello, world!".to_string()
}

use rocket::tokio::time::{sleep, Duration};

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[launch]
fn rocket() -> _ {
    let db = Mutex::new(FlatTable::new("consumers".to_string()));

    println!("Running server..");

    rocket::build()
        .mount("/", routes![index, delay])
        .manage(ConsumerList::new(db))
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
    fn wrong_key_check() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .get(uri!(super::index))
            .header(Header {
                name: "x-api-key".into(),
                value: "wrong-user-id".into(),
            })
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[test]
    fn auth_check() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::index)).dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }
}
