use std::{collections::HashMap, sync::Mutex};

use crate::db::file_db::{get_table_instance, FlatTable};

use self::subscriber_list::SubscriptionList;

pub mod subscriber_list;

pub struct Subscription {
    pub id: u128,
    pub name: String,
    pub status: u8,
    pub price: u128,
    pub quota: u128,
    pub expiry_date: String,
}

impl Subscription {
    pub fn fake(attr: &HashMap<&str, &str>) -> Subscription {
        Subscription {
            id: attr.get("id").unwrap_or(&"1").parse::<u128>().unwrap(),
            name: attr.get("name").unwrap_or(&"default_service").to_string(),
            status: attr.get("status").unwrap_or(&"0").parse::<u8>().unwrap(),
            price: attr.get("price").unwrap_or(&"1").parse::<u128>().unwrap(),
            quota: attr.get("quota").unwrap_or(&"1").parse::<u128>().unwrap(),
            expiry_date: attr
                .get("expiry_date")
                .unwrap_or(&"2001-01-01 00:00:00")
                .to_string(),
        }
    }

    pub fn decrease_quota(&mut self, amount: u128) -> Option<()> {
        if amount > self.quota {
            return None;
        }

        self.quota -= amount;

        Some(())
    }

    pub fn add_quota(&mut self, amount: u128) -> Option<()> {
        self.quota += amount;

        Some(())
    }
}

pub struct Subscriber {
    pub id: u128,
    pub name: String,
    pub subscription: Subscription,
}

impl Subscriber {
    pub fn new(id: u128, name: String, subscription_id: u128) -> Subscriber {
        Subscriber {
            id,
            name,
            subscription: Subscriber::fetch_subscription(
                get_table_instance("subscriptions"),
                subscription_id,
            ),
        }
    }
    pub fn fake(attr: &HashMap<&str, &str>) -> Subscriber {
        Subscriber {
            id: attr.get("id").unwrap_or(&"1").parse::<u128>().unwrap(),
            name: attr.get("name").unwrap_or(&"default_service").to_string(),
            subscription: match attr.get("subscription") {
                Some(subscription_id) => {
                    Subscription::fake(&HashMap::from([("id", *subscription_id)]))
                }
                None => Subscription::fake(&HashMap::new()),
            },
        }
    }

    pub fn fetch_subscription(
        db: Mutex<FlatTable<String, String>>,
        subscription_id: u128,
    ) -> Subscription {
        let subscription_list = SubscriptionList::new(db);
        subscription_list
            .get_by_id(subscription_id)
            .expect(&format!(
                "Subscription with id:{subscription_id} is not found!"
            ))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{Subscriber, Subscription};

    #[test]
    fn test_fetching_subscription() {
        let id = 3;
        let subscriber = Subscriber::fake(&HashMap::from([
            ("id", "1"),
            ("subscription", id.to_string().as_str()),
        ]));
        assert_eq!(subscriber.subscription.id, id)
    }

    #[test]
    fn decrease_quota() {
        let mut subscription: Subscription = Subscription::fake(&HashMap::from([("quota", "128")]));

        subscription.decrease_quota(1);
        subscription.decrease_quota(2);

        assert_eq!(subscription.quota, 125)
    }

    #[test]
    fn do_not_decrease_if_not_enough_quota() {
        let mut subscription: Subscription = Subscription::fake(&HashMap::from([("quota", "1")]));

        match subscription.decrease_quota(2) {
            Some(_) => (),
            None => println!("Not enough credit."),
        }

        assert_eq!(subscription.quota, 1)
    }

    #[test]
    fn add_quota() {
        let mut subscription: Subscription = Subscription::fake(&HashMap::from([("quota", "1")]));

        match subscription.add_quota(100) {
            Some(_) => (),
            None => println!("Couldn't add quota."),
        }

        assert_eq!(subscription.quota, 101)
    }
}
