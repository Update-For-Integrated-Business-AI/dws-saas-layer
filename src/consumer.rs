pub mod consumer_list;

#[derive(Default)]
pub struct Consumer {
    pub id: u32,
    pub quota: u128,
    pub access_token: String,
}

impl Consumer {
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

#[test]
fn decrease_quota() {
    let mut consumer: Consumer = Consumer { quota: 128, ..Default::default() };

    consumer.decrease_quota(1);
    consumer.decrease_quota(2);

    assert_eq!(consumer.quota, 125)
}

#[test]
fn do_not_decrease_if_not_enough_quota() {
    let mut consumer: Consumer = Consumer { quota: 1, ..Default::default() };


    match consumer.decrease_quota(2) {
        Some(_) => (),
        None => println!("Not enough credit."),
    }

    assert_eq!(consumer.quota, 1)
}

#[test]
fn add_quota() {
    let mut consumer: Consumer = Consumer { quota: 1, ..Default::default() };

    match consumer.add_quota(100) {
        Some(_) => (),
        None => println!("Couldn't add quota."),
    }

    assert_eq!(consumer.quota, 101)
}
