pub mod consumer;
pub mod db;
pub mod guards;
pub mod product;
pub mod request;
pub mod service;
pub mod subscriber;
pub use crate::consumer::Consumer;

#[cfg(test)]
mod tests {
    // use super::*;
    // use std::collections::HashMap;

    // #[test]
    // fn successful_request() {
    //     let mut consumer = Consumer::fake(&HashMap::from([("quota", "2")]));

    //     let mut product = product::Product {
    //         price: 1,
    //         requests: 5,
    //     };

    //     // consumer.decrease_quota(product.price);
    //     product.add_request(1);

    //     assert_eq!(consumer.quota, 1);
    //     assert_eq!(product.requests, 6);
    // }
    // #[test]
    // fn failed_request() {
    //     let mut consumer = Consumer::fake(&HashMap::from([("quota", "2")]));

    //     let mut product = product::Product {
    //         price: 5,
    //         requests: 5,
    //     };

    //     match consumer.decrease_quota(product.price) {
    //         Some(_) => product.add_request(1).unwrap(),
    //         None => println!("Not enough credit."),
    //     };

    //     assert_eq!(consumer.quota, 2);
    //     assert_eq!(product.requests, 5);
    // }
}
