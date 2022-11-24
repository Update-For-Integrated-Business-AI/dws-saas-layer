use std::{collections::HashMap, sync::Mutex};

use crate::db::{file_db::FlatTable, ModelAble};

use super::Product;

pub struct ProductList<D> {
    db: Mutex<D>,
    pub products: Vec<Product>,
}

type FlatProductList = ProductList<FlatTable<String, String>>;

impl FlatProductList {
    pub fn new(db: Mutex<FlatTable<String, String>>) -> Self {
        ProductList {
            db,
            products: vec![],
        }
    }

    pub fn get_by_id(&self, id: u128) -> Option<Product> {
        ProductList::get_by_attr::<FlatTable<String, String>, Product>(
            &self.db,
            "id",
            id.to_string(),
        )
    }

    pub fn get_by_slug(&self, slug: &str) -> Option<Product> {
        ProductList::get_by_attr::<FlatTable<String, String>, Product>(
            &self.db,
            "slug",
            slug.to_string(),
        )
    }
}

impl ModelAble<String, String> for FlatProductList {}
impl From<HashMap<String, String>> for Product {
    fn from(map: HashMap<String, String>) -> Self {
        return match (
            map.get("id"),
            map.get("requests"),
            map.get("slug"),
        ) {
            (
                Some(id),
                Some(requests),
                Some(slug),
            ) => Product {
                id: id.parse::<u128>().unwrap(),
                requests: requests.parse::<u128>().unwrap(),
                slug: slug.clone(),
            },
            _ => panic!("Can't convert!"),
        };
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    #[test]
    fn get_product_by_id() {
        let id: u128 = 2;

        let table = "\
        id, slug, requests
        1, product_a, 10
        2, product_b, 11
        "
        .to_string();

        let db = Mutex::new(FlatTable::new_from_string(table));
        let product_list = ProductList::new(db);

        let product = product_list.get_by_id(id).unwrap();

        assert_eq!(product.id, id)
    }

    #[test]
    fn get_product_by_slug() {
        let slug = "product_b";
        let id = 2;

        let table = "\
        id, requests, slug
        1, 10, product_a
        2, 11, product_b
        "
        .to_string();

        let db = Mutex::new(FlatTable::new_from_string(table));
        let product_list = ProductList::new(db);
        let product = product_list.get_by_slug(slug).unwrap();

        assert_eq!(product.id, id);
        assert_eq!(product.slug, slug)
    }
}
