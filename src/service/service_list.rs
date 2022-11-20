use std::{collections::HashMap, sync::Mutex};

use crate::db::{file_db::FlatTable, ModelAble, ToStruct};

use super::Service;

pub struct ServiceList<D> {
    db: Mutex<D>,
    pub services: Vec<Service>,
}

type FlatServiceList = ServiceList<FlatTable<String, String>>;

impl<'a> FlatServiceList {
    pub fn new(db: Mutex<FlatTable<String, String>>) -> Self {
        ServiceList {
            db: db,
            services: vec![],
        }
    }

    pub fn get_by_id(&self, id: u128) -> Option<Service> {
        ServiceList::get_by_attr::<FlatTable<String, String>, FlatServiceList>(
            &self.db,
            "id",
            id.to_string(),
        )
    }

    pub fn get_by_slug(&self, slug: &str) -> Option<Service> {
        ServiceList::get_by_attr::<FlatTable<String, String>, FlatServiceList>(
            &self.db,
            "slug",
            slug.to_string(),
        )
    }
}

impl ModelAble<Service, String, String> for FlatServiceList {}

impl ToStruct<Service, HashMap<String, String>> for FlatServiceList {
    fn convert(data: &HashMap<String, String>) -> Service {
        return match (
            data.get("id"),
            data.get("requests"),
            data.get("name"),
            data.get("slug"),
            data.get("version"),
            data.get("status"),
            data.get("base_url"),
            data.get("price"),
        ) {
            (
                Some(id),
                Some(requests),
                Some(name),
                Some(slug),
                Some(version),
                Some(status),
                Some(base_url),
                Some(price),
            ) => Service {
                id: id.parse::<u128>().unwrap(),
                requests: requests.parse::<u128>().unwrap(),
                name: name.clone(),
                slug: slug.clone(),
                base_url: base_url.clone(),
                version: version.clone(),
                status: status.parse::<u32>().unwrap(),
                price: price.parse::<u128>().unwrap(),
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
    fn get_service_by_id() {
        let id:u128 = 2;

        let table = "\
        id, name, slug, version, status, base_url, price, requests
        1, Service A, service_a, v1.0.0, 1, http://128.0.0.1/123/45, 2, 10 
        2, Service B, service_a, v1.0.0, 2, http://129.0.0.1/123/45, 4, 109 
        "
        .to_string();

        let db = Mutex::new(FlatTable::new_from_string(table));
        let service_list = ServiceList::new(db);

        let service = service_list.get_by_id(id).unwrap();

        assert_eq!(service.id, id)
    }

    #[test]
    fn get_service_by_slug() {
        let slug = "service_b";
        let id = 2;

        let table = "\
        id, name, slug, version, status, base_url, price, requests
        1, Service A, service_a, v1.0.0, 1, http://128.0.0.1/123/45, 2, 10 
        2, Service B, service_b, v1.0.0, 2, http://129.0.0.1/123/45, 4, 109 
        "
        .to_string();

        let db = Mutex::new(FlatTable::new_from_string(table));
        let service_list = ServiceList::new(db);
        let service = service_list.get_by_slug(slug).unwrap();

        assert_eq!(service.id, id);
        assert_eq!(service.slug, slug)
    }
}
