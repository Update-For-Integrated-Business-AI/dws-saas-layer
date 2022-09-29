use std::collections::HashMap;

pub mod consumer_list;

pub struct Consumer {
    pub id: u32,
    pub quota: u128,
    pub access_token: String,
}

impl Consumer {
    pub fn decrease_quota(&mut self, amount: u128) -> Option<()> {
        if amount > self.quota {
            return None;
        }

        self.quota -= amount;

        Some(())
    }

    fn add_quota(&mut self, amount: u128) -> Option<()> {
        self.quota += amount;

        Some(())
    }
}

const ACCESS_TOKEN: &'static str = "A-B-C";

pub mod factory;

#[test]
fn decrease_quota() {
    let mut consumer: Consumer = factory::create_consumer(&HashMap::from([("quota", "128")]));

    consumer.decrease_quota(1);
    consumer.decrease_quota(2);

    assert_eq!(consumer.quota, 125)
}

#[test]
fn do_not_decrease_if_not_enough_quota() {
    let mut consumer: Consumer = Consumer {
        id: 1,
        quota: 1,
        access_token: String::from(ACCESS_TOKEN),
    };

    match consumer.decrease_quota(2) {
        Some(_) => (),
        None => println!("Not enough credit."),
    }

    assert_eq!(consumer.quota, 1)
}

#[test]
fn add_quota() {
    let mut consumer: Consumer = Consumer {
        id: 1,
        quota: 1,
        access_token: String::from(ACCESS_TOKEN),
    };

    match consumer.add_quota(100) {
        Some(_) => (),
        None => println!("Couldn't add quota."),
    }

    assert_eq!(consumer.quota, 101)
}
