use std::{collections::HashMap, sync::Mutex};

use crate::db::{
    file_db::{get_table_instance, FlatTable},
    ModelAble,
};

use super::Request;

pub struct RequestList<D> {
    db: Mutex<D>,
    pub requests: Vec<Request>,
}

type FlatRequestList = RequestList<FlatTable<String, String>>;

impl<'a> FlatRequestList {
    pub fn new(db: Mutex<FlatTable<String, String>>) -> Self {
        RequestList {
            db: db,
            requests: vec![],
        }
    }

    pub fn get_by_id(&self, id: &str) -> Option<Request> {
        RequestList::get_by_attr::<FlatTable<String, String>, Request>(
            &self.db,
            "id",
            id.to_string(),
        )
    }
}

impl ModelAble<String, String> for FlatRequestList {}
impl From<HashMap<String, String>> for Request {
    fn from(map: HashMap<String, String>) -> Self {
        return match (
            map.get("id"),
            map.get("product_slug"),
            map.get("service_slug"),
            map.get("service_version"),
            map.get("url"),
            map.get("status"),
            map.get("price"),
            map.get("service"),
            map.get("consumer"),
        ) {
            (
                Some(id),
                Some(product_slug),
                Some(service_slug),
                Some(service_version),
                Some(url),
                Some(status),
                Some(price),
                Some(service_id),
                Some(consumer_id),
            ) => Request {
                id: id.clone(),
                product_slug: product_slug.clone(),
                service_slug: service_slug.clone(),
                service_version: service_version.clone(),
                url: url.clone(),
                status: status.parse::<u32>().unwrap(),
                price: price.parse::<u128>().unwrap(),
                service: Request::fetch_service(
                    get_table_instance("services"),
                    service_id.parse::<u128>().unwrap(),
                ),
                consumer: Request::fetch_consumer(
                    get_table_instance("consumers"),
                    consumer_id.parse::<u128>().unwrap(),
                ),
            },
            _ => panic!("Can't convert!"),
        };
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    #[test]
    fn get_request_by_id() {
        let id = "UUID-1";

        let table = "\
        id, product_slug, service_slug, service_version, url, status, price, consumer, service
        UUID-1, product_a, service_slug_a, v1.0.0, http://128.0.0.1/123/45, 2, 0, 1, 1
        UUID-2, product_b, service_slug_b, v2.0.0, http://129.0.0.1/123/45, 3, 1, 2, 2
        "
        .to_string();

        let db = Mutex::new(FlatTable::new_from_string(table));
        let request_list = RequestList::new(db);

        let request = request_list.get_by_id(id).unwrap();

        assert_eq!(request.id, id)
    }

}
