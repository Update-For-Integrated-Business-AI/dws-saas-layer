use std::convert::From;
use std::{collections::HashMap, sync::Mutex};

pub type Record<K, V> = HashMap<K, V>;

pub trait Searchable<K, V>
where
    K: Clone,
    V: Clone,
{
    fn find_by(&mut self, attr: &str, value: &str) -> Option<&Record<K, V>>;

    fn get_table_name(&self) -> &str;
}

pub trait ModelAble<K, V>
where
    K: Clone,
    V: Clone,
{
    fn get_by_attr<D: Searchable<K, V>, S: From<Record<K, V>>>(
        db: &Mutex<D>,
        attr: &str,
        value: String,
    ) -> Option<S> {
        let mut lock = db.lock().expect("lock db");
        lock.find_by(attr, value.as_str()).map(|record| S::from(record.clone()))
    }
}

pub trait ToStruct<T, K> {
    fn convert(data: &K) -> T;
}

pub mod file_db {
    use super::*;
    use std::{collections::HashMap, fs};

    fn read_from_file(table: &str) -> String {
        fs::read_to_string(format!("db/{}_table.txt", table))
            .expect("Should have been able to read the file")
    }

    fn get_column_names(content: &str) -> Vec<String> {
        return match content.lines().next() {
            Some(first_line) => first_line
                .split(',')
                .map(|f| String::from(f.trim()))
                .collect::<Vec<String>>(),
            None => panic!("Table has no columns"),
        };
    }

    fn get_records(content: &str) -> Vec<Vec<String>> {
        let mut rows = vec![];
        for line in content.lines().skip(1) {
            // skip header line (header)
            let row = match Some(line) {
                Some(l) => l
                    .split(',')
                    .map(|f| String::from(f.trim()))
                    .collect::<Vec<String>>(),
                None => vec![], // add empty row if the database is empty
            };
            rows.push(row);
        }
        rows
    }

    fn create_flat_table(
        columns: Vec<String>,
        rows: Vec<Vec<String>>,
    ) -> Vec<Record<String, String>> {
        let mut objects = vec![];
        for row in rows.iter() {
            let mut row_object = HashMap::new();
            for (i, column) in row.iter().enumerate() {
                if let Some(column_name) = columns.get(i) {
                    row_object.insert(column_name.clone(), column.clone());
                }
            }
            objects.push(row_object);
        }
        objects
    }

    pub fn read(table_name: &str) -> Vec<Record<String, String>> {
        let table = read_from_file(table_name);
        let columns = get_column_names(&table);
        let rows = get_records(&table);
        create_flat_table(columns, rows)
    }

    pub fn read_from_string(content: &str) -> Vec<Record<String, String>> {
        let columns = get_column_names(content);
        let rows = get_records(content);
        create_flat_table(columns, rows)
    }
    #[derive(Clone)]
    pub struct FlatTable<K, V> {
        pub table_name: String,
        pub items: Vec<Record<K, V>>,
        source: u8,
        raw: String,
    }
    pub fn get_table_instance(db_name: &str) -> Mutex<FlatTable<String, String>> {
        Mutex::new(FlatTable::new(db_name.to_string()))
    }
    impl FlatTable<String, String> {
        pub fn new(table_name: String) -> Self {
            FlatTable {
                table_name,
                items: vec![],
                source: 1,
                raw: String::new(),
            }
        }

        pub fn new_from_string(contents: String) -> Self {
            FlatTable {
                table_name: "from_string".to_string(),
                items: read_from_string(&contents),
                source: 2,
                raw: contents,
            }
        }

        pub fn refresh(&mut self) -> &Self {
            self.items = match self.source {
                1 => file_db::read(self.table_name.as_str()),
                2 => read_from_string(&self.raw),
                _ => panic!("Invalid source!"),
            };
            self
        }
    }

    impl super::Searchable<String, String> for FlatTable<String, String> {
        fn find_by(&mut self, attr: &str, value: &str) -> Option<&Record<String, String>> {
            self.refresh();
            self.items.iter().find(|record| match record.get(attr) {
                Some(a) => a == value,
                None => false,
            })
        }

        fn get_table_name(&self) -> &str {
            &self.table_name
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_extracting_column_names_from_file() {
            let table = "\
            column1, column2, column3
            row1_value1, row1_value2, row1_value3
            row2_value1, row2_value2, row2_value3";

            let columns = get_column_names(&String::from(table));

            assert_eq!(columns, vec!["column1", "column2", "column3"])
        }

        #[test]
        fn test_extracting_rows_from_file() {
            let table = "\
            column1, column2, column3
            row1_value1, row1_value2, row1_value3
            row2_value1, row2_value2, row2_value3";

            let rows = get_records(&String::from(table));

            assert_eq!(
                rows,
                vec![
                    vec!["row1_value1", "row1_value2", "row1_value3"],
                    vec!["row2_value1", "row2_value2", "row2_value3"],
                ]
            )
        }

        #[test]
        fn test_creating_flat_table() {
            let table = String::from(
                "\
            column1, column2, column3
            row1_value1, row1_value2, row1_value3
            row2_value1, row2_value2, row2_value3",
            );

            let columns = get_column_names(&table);
            let rows = get_records(&table);
            let data_object = create_flat_table(columns, rows);
            assert_eq!(
                data_object,
                vec![
                    HashMap::from([
                        ("column1".to_string(), "row1_value1".to_string()),
                        ("column2".to_string(), "row1_value2".to_string()),
                        ("column3".to_string(), "row1_value3".to_string())
                    ]),
                    HashMap::from([
                        ("column1".to_string(), "row2_value1".to_string()),
                        ("column2".to_string(), "row2_value2".to_string()),
                        ("column3".to_string(), "row2_value3".to_string())
                    ])
                ]
            )
        }
        #[test]
        fn test_finding_by_attribute_and_value() {
            let table = String::from(
                "\
            column1, column2, column3
            row1_value1, row1_value2, row1_value3
            row2_value1, row2_value2, row2_value3",
            );

            let mut flat_table = FlatTable {
                raw: table.clone(),
                table_name: "N\\A".to_string(),
                items: read_from_string(&table),
                source: 2,
            };

            if let Some(record) = flat_table.find_by("column2", "row2_value2") {
                assert_eq!(
                    record,
                    &HashMap::from([
                        ("column1".to_string(), "row2_value1".to_string()),
                        ("column2".to_string(), "row2_value2".to_string()),
                        ("column3".to_string(), "row2_value3".to_string())
                    ])
                )
            }
        }
    }
}
