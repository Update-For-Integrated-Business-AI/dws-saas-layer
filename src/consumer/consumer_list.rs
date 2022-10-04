use super::{Consumer};

pub struct ConsumerList {
    pub consumers: Vec<Consumer>,
}

impl ConsumerList {
    fn get_by_id(&self, id: u32) -> Option<&Consumer> {
        return self.consumers.iter().find(|c| c.id == id);
    }

    fn get_by_access_token(&self, access_token: &str) -> Option<&Consumer> {
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
        Consumer {
            id: 1,
            ..Default::default()
        },
        Consumer {
            id: 2,
            ..Default::default()
        },
        Consumer {
            id: 3,
            ..Default::default()
        },
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
        Consumer {
            id: 1,
            access_token: String::from("A-1"),
            ..Default::default()
        },
        Consumer {
            id: 2,
            access_token: String::from("A-2"),
            ..Default::default()
        },
        Consumer {
            id: 3,
            access_token: String::from("A-3"),
            ..Default::default()
        },
    ];

    let consumer_list = ConsumerList { consumers };

    let consumer = consumer_list.get_by_access_token(access_token).unwrap();

    assert_eq!(consumer.id, id)
}
