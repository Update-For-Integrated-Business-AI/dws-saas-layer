use std::collections::HashMap;

pub trait Searchable<K, V> {
    fn find_by(&self, attr: &str, value: &str) -> Option<&HashMap<K, V>>;
}

mod file_db {
    use super::*;
    use std::{collections::HashMap, fs};

    fn read_from_file(table: &str) -> String {
        return fs::read_to_string(format!("db/{}_table.txt", table))
            .expect("Should have been able to read the file");
    }

    fn get_column_names(content: &String) -> Vec<String> {
        return match content.lines().next() {
            Some(first_line) => first_line
                .split(',')
                .map(|f| String::from(f.trim()))
                .collect::<Vec<String>>(),
            None => panic!("Table has no columns"),
        };
    }

    fn get_records(content: &String) -> Vec<Vec<String>> {
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
    ) -> Vec<HashMap<String, String>> {
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

    pub fn read(table_name: &str) -> Vec<HashMap<String, String>> {
        let table = read_from_file(table_name);
        let columns = get_column_names(&table);
        let rows = get_records(&table);
        return create_flat_table(columns, rows);
    }

    pub fn read_from_string(content: &String) -> Vec<HashMap<String, String>> {
        let columns = get_column_names(content);
        let rows = get_records(content);
        return create_flat_table(columns, rows);
    }

    struct FlatTable<K, V> {
        items: Vec<HashMap<K, V>>,
    }

    impl FlatTable<String, String> {
        fn new(&mut self, table_name: &str) {
            self.items = file_db::read(table_name);
        }
    }

    impl super::Searchable<String, String> for FlatTable<String, String> {
        fn find_by(&self, attr: &str, value: &str) -> Option<&HashMap<String, String>> {
            self.items.iter().find(|record| match record.get(attr) {
                Some(a) => a == value,
                None => false,
            })
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

            let flat_table = FlatTable {
                items: read_from_string(&table)
            };

            if let Some(record) = flat_table.find_by("column2", "row2_value2") {
                assert_eq!(record, &HashMap::from([
                    ("column1".to_string(), "row2_value1".to_string()),
                    ("column2".to_string(), "row2_value2".to_string()),
                    ("column3".to_string(), "row2_value3".to_string())
                ]))
            }


        }
    }
}
