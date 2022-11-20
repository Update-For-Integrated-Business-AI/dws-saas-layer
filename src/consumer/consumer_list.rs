use std::collections::HashMap;

use crate::db::{file_db::FlatTable, ToStruct, ModelAble};

use super::Consumer;

pub struct ConsumerList<D> {
    db: D,
    pub consumers: Vec<Consumer>,
}

type FlatTableConsumerList = ConsumerList<FlatTable<String, String>>;

impl FlatTableConsumerList {
    pub fn new(db: FlatTable<String, String>) -> Self {
        ConsumerList {
            db: db,
            consumers: vec![],
        }
    }

    pub fn get_by_id(&self, id: u32) -> Option<Consumer> {
        ConsumerList::get_by_attr::<FlatTable<String, String>, FlatTableConsumerList>(
            self.db.clone(),
            "id",
            id.to_string(),
        )
    }

    pub fn get_by_access_token(&self, access_token: &str) -> Option<Consumer> {
        ConsumerList::get_by_attr::<FlatTable<String, String>, FlatTableConsumerList>(
            self.db.clone(),
            "access_token",
            access_token.to_string(),
        )
    }
}

impl ModelAble<Consumer, String, String> for FlatTableConsumerList {}


impl ToStruct<Consumer, HashMap<String, String>> for FlatTableConsumerList {
    fn convert(data: &HashMap<String, String>) -> Consumer {
        return match (data.get("id"), data.get("quota"), data.get("access_token")) {
            (Some(id), Some(quota), Some(access_token)) => Consumer {
                id: id.parse::<u32>().unwrap(),
                quota: quota.parse::<u128>().unwrap(),
                access_token: access_token.clone(),
            },
            _ => panic!("Can't convert!"),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_consumer_by_id() {
        let id = 2;

        let table = "\
        id, quota, access_token
        1, 120, row1_value3
        2, 130, row2_value3\
        "
        .to_string();

        let db = FlatTable::new_from_string(table);
        let consumer_list = ConsumerList::new(db);

        let consumer = consumer_list.get_by_id(id).unwrap();

        assert_eq!(consumer.id, id)
    }

    #[test]
    fn get_consumer_by_access_token() {
        let access_token = "A-2";
        let id = 2;

        let table = "\
        id, quota, access_token
        1, 120, A-1
        2, 130, A-2\
        "
        .to_string();

        let db = FlatTable::new_from_string(table);
        let consumer_list = ConsumerList::new(db);
        let consumer = consumer_list.get_by_access_token(access_token).unwrap();

        assert_eq!(consumer.id, id);
        assert_eq!(consumer.access_token, access_token)
    }
}
