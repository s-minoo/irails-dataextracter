use std::collections::{BTreeMap, HashSet};

use log::trace;

use crate::data_type::Record;
use crate::error::CleanResult;

pub fn flatten_json(
    nldjson: &str,
    filter_set: &HashSet<String>,
) -> CleanResult<Record> {
    let binding = json::parse(nldjson)?;
    match binding {
        json::JsonValue::Object(object) => {
            if object.get("error").is_some() {
                return Err(crate::error::ParseError::Data(format!(
                    "The query caused an erorr: {}",
                    nldjson
                )));
            }
            if let Some(querytype) = object.get("querytype") {
                if filter_set.contains(&querytype.to_string().to_lowercase()) {
                    let parsed_entries: BTreeMap<_, _> = object
                        .iter()
                        .map(|(field, value)| {
                            (field.to_string(), value.to_string())
                        })
                        .collect();
                    return Ok(parsed_entries.into());
                }
            }
            Err(crate::error::ParseError::Data(
                "Input query type is filtered out ".to_string(),
            ))
        }
        _ => {
            Err(crate::error::ParseError::Data(format!(
                "Input string is not a valid JSONObject: {} ",
                nldjson
            )))
        }
    }
}
