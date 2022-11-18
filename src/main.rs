#[macro_use]
extern crate rocket;

use std::collections::HashMap;

use sass_layer::consumer::{consumer_list::ConsumerList, Consumer};

use sass_layer::guards::{HostHeader, ApiKey};



#[get("/")]
fn index(_key: ApiKey, _host: HostHeader, ) -> String{
    format!("Hello, world!")
}

use rocket::tokio::time::{sleep, Duration};

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[launch]
fn rocket() -> _ {
    let consumers = vec![Consumer::fake(&HashMap::from([
        ("id", "1"),
        ("access_token", "user-1"),
    ]))];

    rocket::build()
        .mount("/", routes![index, delay])
        .manage(ConsumerList { consumers })
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
