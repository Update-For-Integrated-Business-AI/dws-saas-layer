use std::{collections::HashMap, sync::Mutex};

use crate::{
    consumer::consumer_list::ConsumerList,
    db::file_db::{get_table_instance, FlatTable},
    service::{service_list::ServiceList, Service},
    Consumer,
};

pub mod request_list;

pub struct Request {
    pub id: String,
    pub consumer: Consumer,
    pub service: Service,
    pub product_slug: String,
    pub service_slug: String,
    pub service_version: String,
    pub url: String,
    pub status: u32,
    pub price: u128
}

impl Request {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        product_slug: String,
        service_slug: String,
        service_version: String,
        url: String,
        service_id: u128,
        consumer_id: u128,
        status: u32,
        price: u128,
    ) -> Request {
        Request {
            id,
            product_slug,
            service_slug,
            service_version,
            url,
            status,
            price,
            service: Request::fetch_service(get_table_instance("services"), service_id),
            consumer: Request::fetch_consumer(get_table_instance("consumers"), consumer_id),
        }
    }

    pub fn fake(attr: &HashMap<&str, &str>) -> Request {
        Request {
            id: attr.get("id").unwrap_or(&"UUID").to_string(),
            product_slug: attr
                .get("product_slug")
                .unwrap_or(&"default_product_service")
                .to_string(),
            service_slug: attr
                .get("service_slug")
                .unwrap_or(&"service_service_slug")
                .to_string(),
            service_version: attr.get("service_version").unwrap_or(&"v0.0.1").to_string(),
            url: attr.get("url").unwrap_or(&"https://A-B-C.com").to_string(),
            status: attr.get("status").unwrap_or(&"0").parse::<u32>().unwrap(),
            price: attr.get("price").unwrap_or(&"2").parse::<u128>().unwrap(),
            service: match attr.get("service") {
                Some(service_id) => Service::fake(&HashMap::from([("id", *service_id)])),
                None => Service::fake(&HashMap::new()),
            },
            consumer: match attr.get("consumer") {
                Some(consumer_id) => Consumer::fake(&HashMap::from([("id", *consumer_id)])),
                None => Consumer::fake(&HashMap::new()),
            },
        }
    }

    pub fn fetch_consumer(db: Mutex<FlatTable<String, String>>, consumer_id: u128) -> Consumer {
        let consumer_list = ConsumerList::new(db);
        consumer_list
            .get_by_id(consumer_id)
            .unwrap_or_else(|| panic!("Consumer with id:{consumer_id} is not found!"))
    }

    pub fn fetch_service(db: Mutex<FlatTable<String, String>>, service_id: u128) -> Service {
        let service_list = ServiceList::new(db);
        service_list
            .get_by_id(service_id)
            .unwrap_or_else(|| panic!("Service with id:{service_id} is not found!"))
    }
}

#[cfg(test)]
mod tests {}
