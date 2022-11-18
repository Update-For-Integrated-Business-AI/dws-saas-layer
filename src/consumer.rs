use std::collections::HashMap;

pub mod consumer_list;

pub struct Consumer {
    pub id: u32,
    pub quota: u128,
    pub access_token: String,
}

impl Consumer {
    pub fn new(attr: &HashMap<&str, &str>) -> Consumer {
        Consumer {
            id: attr.get("id").unwrap_or(&"1").parse::<u32>().unwrap(),
            quota: attr.get("quota").unwrap_or(&"10").parse::<u128>().unwrap(),
            access_token: attr.get("access_token").unwrap_or(&"A-B-C").to_string(),
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decrease_quota() {
        let mut consumer: Consumer = Consumer::new(&HashMap::from([("quota", "128")]));

        consumer.decrease_quota(1);
        consumer.decrease_quota(2);

        assert_eq!(consumer.quota, 125)
    }

    #[test]
    fn do_not_decrease_if_not_enough_quota() {
        let mut consumer: Consumer = Consumer::new(&HashMap::from([("quota", "1")]));

        match consumer.decrease_quota(2) {
            Some(_) => (),
            None => println!("Not enough credit."),
        }

        assert_eq!(consumer.quota, 1)
    }

    #[test]
    fn add_quota() {
        let mut consumer: Consumer = Consumer::new(&HashMap::from([("quota", "1")]));

        match consumer.add_quota(100) {
            Some(_) => (),
            None => println!("Couldn't add quota."),
        }

        assert_eq!(consumer.quota, 101)
    }
}
