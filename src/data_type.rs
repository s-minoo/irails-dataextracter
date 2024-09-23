use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Record {
    pub map: BTreeMap<String, String>,
}

impl Record {
    pub fn to_headless_string(&self) -> String {
        self.map
            .values()
            .into_iter()
            .map(|e| e.as_ref())
            .collect::<Vec<_>>()
            .join(",")
    }

    pub fn get_type(&self) -> String {
        self.map.get("type").unwrap().to_string()
    }
}

impl ToString for Record {
    fn to_string(&self) -> String {
        let header = self
            .map
            .keys()
            .into_iter()
            .map(|e| e.as_ref())
            .collect::<Vec<_>>()
            .join(",");

        header + "\n" + &self.to_headless_string()
    }
}

impl From<BTreeMap<String, String>> for Record {
    fn from(value: BTreeMap<String, String>) -> Self {
        Record { map: value }
    }
}
