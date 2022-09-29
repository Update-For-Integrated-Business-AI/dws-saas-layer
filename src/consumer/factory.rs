use std::collections::HashMap;

use super::Consumer;

pub fn create_consumer(attr: &HashMap<&str, &str>) -> Consumer {
    Consumer {
        id: attr.get("id").unwrap_or(&"1").parse::<u32>().unwrap(),
        quota: attr.get("quota").unwrap_or(&"10").parse::<u128>().unwrap(),
        access_token: attr.get("access_token").unwrap_or(&"A-B-C").to_string(),
    }
}
