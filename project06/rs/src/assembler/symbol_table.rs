use std::collections::HashMap;

pub struct SymbolTable {
    curr_addr: u32,
    pub entries: HashMap<String, u32>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            curr_addr: 16,
            entries: HashMap::new(),
        }
    }

    pub fn get(&self, symbol: &str) -> Result<&u32, String> {
        let table_entry = self.entries.get(symbol);
        if table_entry.is_none() {
            return Err(format!(
                "Unable to retieve symbol from symbol table: {}",
                symbol
            ));
        }

        Ok(table_entry.unwrap())
    }

    pub fn insert(&mut self, symbol: &str) {
        let table_entry = self.entries.get(symbol);
        if table_entry.is_none() {
            self.entries.insert(String::from(symbol), self.curr_addr);
            self.curr_addr += 1;
        }
    }

    pub fn contains(&mut self, symbol: &str) -> bool {
        self.entries.contains_key(symbol)
    }
}

#[cfg(test)]
mod tests {
    mod get {
        use super::super::*;

        #[test]
        fn test_get_exists() {
            let mut table = SymbolTable::new();
            table.entries.insert(String::from("myvar"), 999);

            let entry = table.get("myvar");
            assert!(entry.is_ok());
            assert_eq!(entry.unwrap(), &999);
        }
        #[test]
        fn test_get_no_exists() {
            let table = SymbolTable::new();
            let entry = table.get("myvar");
            assert!(entry.is_err());
        }
    }
    mod insert {
        use super::super::*;

        #[test]
        fn test_insert_new() {
            let mut table = SymbolTable::new();
            table.insert("myvar");
            assert_eq!(table.entries.get("myvar"), Some(&16));
        }
        #[test]
        fn test_insert_exists() {
            let mut table = SymbolTable::new();
            table.insert("myvar");
            table.insert("myvar");
            assert_eq!(table.entries.get("myvar"), Some(&16));
        }
    }
    mod contains {
        use super::super::*;

        #[test]
        fn test_contains() {
            let mut table = SymbolTable::new();
            table.entries.insert(String::from("myvar"), 16);
            assert!(table.contains("myvar"));
        }
        #[test]
        fn test_does_not_contain() {
            let mut table = SymbolTable::new();
            assert!(!table.contains("myvar"));
        }
    }
}
