use std::collections::HashMap;

pub mod consumer_list;

#[derive(Debug, Clone)]
pub struct Consumer {
    pub id: u32,
    pub access_token: String,
}

impl Consumer {
    pub fn fake(attr: &HashMap<&str, &str>) -> Consumer {
        Consumer {
            id: attr.get("id").unwrap_or(&"1").parse::<u32>().unwrap(),
            access_token: attr.get("access_token").unwrap_or(&"A-B-C").to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    
}
