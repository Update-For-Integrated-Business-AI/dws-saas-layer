use std::{collections::HashMap, sync::Mutex};

use crate::{db::{Record, file_db::FlatTable, ModelAble}};

use super::{Subscriber, Subscription};

pub type FlatSubscriptionList = SubscriptionList<FlatTable<String, String>>;
pub struct SubscriptionList<D> {
    db: Mutex<D>,
    pub subscriptions: Vec<Subscription>,
}

impl FlatSubscriptionList {
    pub fn new(db: Mutex<FlatTable<String, String>>) -> Self {
        SubscriptionList {
            db,
            subscriptions: vec![],
        }
    }

    pub fn get_by_id(&self, id: u128) -> Option<Subscription> {
        SubscriptionList::get_by_attr::<FlatTable<String, String>, Subscription>(
            &self.db,
            "id",
            id.to_string(),
        )
    }
}

impl ModelAble<String, String> for FlatSubscriptionList {}

impl From<Record<String, String>> for Subscription {
    fn from(map: Record<String, String>) -> Self {
        return match (
            map.get("id"),
            map.get("name"),
            map.get("status"),
            map.get("price"),
            map.get("quota"),
            map.get("expiry_date"),
        ) {
            (Some(id), Some(name), Some(status), Some(price), Some(quota), Some(expiry_date)) => Subscription {
                id: id.parse::<u128>().unwrap(),
                name: name.clone(),
                status: status.parse::<u8>().unwrap(),
                price: price.parse::<u128>().unwrap(),
                quota: quota.parse::<u128>().unwrap(),
                expiry_date: expiry_date.clone(),
            },
            _ => panic!("Can't convert! Invalid Structure. "),
        };
    }
}


pub type FlatSubscriberList = SubscriberList<FlatTable<String, String>>;

pub struct SubscriberList<D> {
    db: Mutex<D>,
    pub subscribers: Vec<Subscriber>,
}

impl FlatSubscriberList {
    pub fn new(db: Mutex<FlatTable<String, String>>) -> Self {
        SubscriberList {
            db,
            subscribers: vec![],
        }
    }

    pub fn get_by_id(&self, id: u128) -> Option<Subscriber> {
        SubscriberList::get_by_attr::<FlatTable<String, String>, Subscriber>(
            &self.db,
            "id",
            id.to_string(),
        )
    }
}

impl ModelAble<String, String> for FlatSubscriberList {}

impl From<Record<String, String>> for Subscriber {
    fn from(map: Record<String, String>) -> Self {
        return match (
            map.get("id"),
            map.get("name"),
            map.get("subscription"),
        ) {
            (Some(id), Some(name), Some(_subscription)) => Subscriber {
                id: id.parse::<u128>().unwrap(),
                name: name.clone(),
                subscription: super::Subscription::fake(&HashMap::new()),
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
    fn get_subscriber_by_id() {
        let id: u128 = 2;

        let table = "\
        id, name, subscription
        1, Subscriber A, 1
        2, Subscriber B, 2
        "
        .to_string();

        let db = Mutex::new(FlatTable::new_from_string(table));
        let subscriber_list = SubscriberList::new(db);

        let subscriber = subscriber_list.get_by_id(id).unwrap();

        assert_eq!(subscriber.id, id)
    }

    #[test]
    fn get_subscription_by_id() {
        let id: u128 = 2;

        let table = "\
        id, name, status, price, quota, expiry_date
        1, Startup 500, 1, 10000, 50, 2022-10-01 00:00:00
        2, Golden 50, 2, 50000, 10, 2022-10-01 00:00:00
        "
            .to_string();

        let db = Mutex::new(FlatTable::new_from_string(table));
        let subscription_list = SubscriptionList::new(db);

        let subscription = subscription_list.get_by_id(id).unwrap();

        assert_eq!(subscription.id, id)
    }
}