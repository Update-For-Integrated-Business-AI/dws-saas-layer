use std::collections::HashMap;

pub mod service_list;

pub struct Service {
    pub id: u128,
    pub name: String,
    pub requests: u128,
    pub slug: String,
    pub status: u32,
    pub version: String,
    pub base_url: String,
    pub price: u128
}

impl Service {
    pub fn fake(attr: &HashMap<&str, &str>) -> Service {
        Service {
            id: attr.get("id").unwrap_or(&"1").parse::<u128>().unwrap(),
            name: attr.get("name").unwrap_or(&"default_service").to_string(),
            requests: attr.get("quota").unwrap_or(&"10").parse::<u128>().unwrap(),
            slug: attr.get("slug").unwrap_or(&"service_slug").to_string(),
            status: attr.get("status").unwrap_or(&"0").parse::<u32>().unwrap(),
            version: attr.get("version").unwrap_or(&"v0.0.1").to_string(),
            base_url: attr.get("base_url").unwrap_or(&"A-B-C").to_string(),
            price: attr.get("price").unwrap_or(&"1").parse::<u128>().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {

}