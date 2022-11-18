pub struct Product {
    pub price: u128,
    pub requests: u128,
}

impl Product {
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
            price: 1,
            requests: 5,
        };

        product.add_request(1);

        assert_eq!(product.requests, 6);
    }
}
