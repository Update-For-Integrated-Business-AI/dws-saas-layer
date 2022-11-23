use std::collections::HashMap;

pub mod product_list;
pub struct Product {
    pub id: u128,
    pub slug: String,
    pub requests: u128,
}

impl Product {
    pub fn fake(attr: &HashMap<&str, &str>) -> Product {
        Product {
            id: attr.get("price").unwrap_or(&"1").parse::<u128>().unwrap(),
            slug: attr.get("slug").unwrap_or(&"default_product").to_string(),
            requests: attr
                .get("requests")
                .unwrap_or(&"10")
                .parse::<u128>()
                .unwrap(),
        }
    }

    pub fn add_request(&mut self, amount: u128) -> Option<()> {
        self.requests += amount;

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_request() {
        let mut product = Product {
            id: 1,
            requests: 5,
            slug: String::from("product"),
        };

        product.add_request(1);

        assert_eq!(product.requests, 6);
    }
}
