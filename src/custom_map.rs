use std::collections::HashMap;

enum AllowedData {
    S(String),
    I(i8)
}

struct CustomMap {
    body: HashMap<String, AllowedData>
}

impl CustomMap {
    fn new() -> CustomMap {
        return CustomMap{
            body: HashMap::new()
        }
    }

    fn get(&self, key: &str) -> &AllowedData {
        return self.body.get(key).unwrap()
    }

    fn insert(&mut self, key: &str, value: AllowedData) -> () {
        self.body.insert(key.to_string(), value);
    }

    fn display(&self, key: &str) -> () {
        match self.get(key) {
            AllowedData::I(value) => println!("{}", value),
            AllowedData::S(value) => println!("{}", value)
        }
    }
}