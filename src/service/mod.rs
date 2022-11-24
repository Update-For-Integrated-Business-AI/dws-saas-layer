use std::{collections::HashMap, sync::Mutex};

use crate::{
    db::file_db::{get_table_instance, FlatTable},
    product::{product_list::ProductList, Product},
};

pub mod service_list;

pub struct Service {
    pub id: u128,
    pub name: String,
    pub requests: u128,
    pub slug: String,
    pub status: u32,
    pub version: String,
    pub base_url: String,
    pub price: u128,
    pub product: Product,
}

impl Service {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        id: u128,
        name: String,
        requests: u128,
        slug: String,
        status: u32,
        version: String,
        base_url: String,
        price: u128,
        product_id: u128,
    ) -> Service {
        Service {
            id,
            name,
            requests,
            slug,
            status,
            version,
            base_url,
            price,
            product: Service::fetch_product(get_table_instance("products"), product_id),
        }
    }

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
            product: match attr.get("product") {
                Some(product_id) => Product::fake(&HashMap::from([("id", *product_id)])),
                None => Product::fake(&HashMap::new()),
            },
        }
    }

    pub fn fetch_product(db: Mutex<FlatTable<String, String>>, product_id: u128) -> Product {
        let product_list = ProductList::new(db);
        product_list
            .get_by_id(product_id)
            .unwrap_or_else(|| panic!("Product with id:{product_id} is not found!"))
    }
}

#[cfg(test)]
mod tests {}
