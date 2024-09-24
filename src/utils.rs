use std::collections::BTreeMap;

use crate::data_type::Record;
use crate::error::CleanResult;

pub fn flatten_json(nldjson: &str) -> CleanResult<Record> {
    let binding = json::parse(nldjson)?;
    if binding.has_key("error") {
        return Err(crate::error::ParseError::DataError(format!(
            "The query caused an erorr: {}",
            nldjson
        )));
    }

    let parsed_entries: BTreeMap<_, _> = binding
        .entries()
        .map(|(field, value)| (field.to_string(), value.to_string()))
        .collect();
    Ok(parsed_entries.into())
}
