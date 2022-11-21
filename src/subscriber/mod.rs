use std::collections::HashMap;

pub mod subscriber_list;

pub struct Subscription {
    pub id: u128,
    pub name: String,
    pub status: u8,
    pub price: u128,
    pub quota: u128,
}

impl Subscription {
    pub fn fake(attr: &HashMap<&str, &str>) -> Subscription {
        Subscription {
            id: attr.get("id").unwrap_or(&"1").parse::<u128>().unwrap(),
            name: attr.get("name").unwrap_or(&"default_service").to_string(),
            status: attr.get("status").unwrap_or(&"0").parse::<u8>().unwrap(),
            price: attr.get("price").unwrap_or(&"1").parse::<u128>().unwrap(),
            quota: attr.get("quota").unwrap_or(&"1").parse::<u128>().unwrap(),
        }
    }
}

pub struct Subscriber {
    pub id: u128,
    pub name: String,
    pub subscription: Subscription,
}

impl Subscriber {
    pub fn fake(attr: &HashMap<&str, &str>) -> Subscriber {
        Subscriber {
            id: attr.get("id").unwrap_or(&"1").parse::<u128>().unwrap(),
            name: attr.get("name").unwrap_or(&"default_service").to_string(),
            subscription: Subscription::fake(&HashMap::new()),
        }
    }
}

#[cfg(test)]
mod tests {

}