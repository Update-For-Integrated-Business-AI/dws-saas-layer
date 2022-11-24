use std::{collections::HashMap, sync::Mutex};

use crate::{
    db::file_db::{get_table_instance, FlatTable},
    subscriber::{subscriber_list::SubscriberList, Subscriber},
};

pub mod consumer_list;

#[derive(Debug, Clone)]
pub struct Consumer {
    pub id: u128,
    pub access_token: String,
    pub subscriber: Subscriber,
}

impl Consumer {
    pub fn new(id: u128, access_token: String, subscriber_id: u128) -> Consumer {
        Consumer {
            id,
            subscriber: Consumer::fetch_subscriber(
                get_table_instance("subscribers"),
                subscriber_id,
            ),
            access_token,
        }
    }

    pub fn fake(attr: &HashMap<&str, &str>) -> Consumer {
        Consumer {
            id: attr.get("id").unwrap_or(&"1").parse::<u128>().unwrap(),
            subscriber: match attr.get("subscriber") {
                Some(subscriber_id) => Subscriber::fake(&HashMap::from([("id", *subscriber_id)])),
                None => Subscriber::fake(&HashMap::new()),
            },
            access_token: attr.get("access_token").unwrap_or(&"A-B-C").to_string(),
        }
    }

    pub fn fetch_subscriber(
        db: Mutex<FlatTable<String, String>>,
        subscriber_id: u128,
    ) -> Subscriber {
        let subscriber_list = SubscriberList::new(db);
        subscriber_list
            .get_by_id(subscriber_id)
            .unwrap_or_else(|| panic!("Subscriber with id:{subscriber_id} is not found!"))
    }
}

#[cfg(test)]
mod tests {}
