use std::collections::BTreeMap;

#[derive(Debug)]
pub struct ORResult {
    pub header: Vec<String>,
    pub header_metadata: BTreeMap<String, String>,
    pub result_set: Vec<Vec<String>>,
}

impl ORResult {
    pub fn new(
        header_metadata: BTreeMap<String, String>,
        header: Vec<String>,
        result_set: Vec<Vec<String>>,
    ) -> Self {
        Self {
            header,
            header_metadata,
            result_set,
        }
    }
}
