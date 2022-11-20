use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use crate::consumer::consumer_list::ConsumerList;
use crate::db::file_db::FlatTable;

#[derive(Debug)]
pub struct HostHeader<'a>(pub &'a str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HostHeader<'r> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Host") {
            Some(h) => Outcome::Success(HostHeader(h)),
            None => Outcome::Success(HostHeader("")),
        }
    }
}



#[derive(Debug)]
pub struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        async fn is_valid(req: &Request<'_>, key: &str) -> bool {
            let consumer_list = req.guard::<&State<ConsumerList<FlatTable<String, String>>>>().await.unwrap();
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
mod tests {

}