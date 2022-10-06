use std::collections::HashMap;

use super::{factory, Consumer};

pub struct ConsumerList {
    pub consumers: Vec<Consumer>,
}

impl ConsumerList {
    pub fn get_by_id(&self, id: u32) -> Option<&Consumer> {
        return self.consumers.iter().find(|c| c.id == id);
    }

    pub fn get_by_access_token(&self, access_token: &str) -> Option<&Consumer> {
        return self
            .consumers
            .iter()
            .find(|c| c.access_token == access_token);
    }
}

#[test]
fn get_consumer_by_id() {
    let id = 2;

    let consumers = vec![
        factory::create_consumer(&HashMap::from([("id", "1")])),
        factory::create_consumer(&HashMap::from([("id", "2")])),
        factory::create_consumer(&HashMap::from([("id", "3")])),
    ];

    let consumer_list = ConsumerList { consumers };

    let consumer = consumer_list.get_by_id(id).unwrap();

    assert_eq!(consumer.id, id)
}

#[test]
fn get_consumer_by_access_token() {
    let access_token = "A-2";
    let id = 2;

    let consumers = vec![
        factory::create_consumer(&HashMap::from([("id", "1"), ("access_token", "A-1")])),
        factory::create_consumer(&HashMap::from([("id", "2"), ("access_token", "A-2")])),
        factory::create_consumer(&HashMap::from([("id", "3"), ("access_token", "A-3")])),
    ];

    let consumer_list = ConsumerList { consumers };

    let consumer = consumer_list.get_by_access_token(access_token).unwrap();

    assert_eq!(consumer.id, id)
}
