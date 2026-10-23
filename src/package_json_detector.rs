use serde_json::Value;

use crate::occurrence::Occurrence;

const SECTIONS: [&str; 2] = ["engines", "volta"];

// Values in `engines.node` and `volta.node`. Leaves the file's shape alone and only finds value positions
pub fn find(content: &str) -> Result<Vec<Occurrence>, String> {
    let document: Value = serde_json::from_str(content).map_err(|error| error.to_string())?;
    let found = SECTIONS.iter().filter_map(|section| section_occurrence(content, &document, section));
    Ok(found.collect())
}

fn section_occurrence(content: &str, document: &Value, section: &str) -> Option<Occurrence> {
    let value = document.get(section).and_then(|value| value.get("node")).and_then(Value::as_str)?;
    let slice = locate_value(content, section, value)?;
    Some(Occurrence::from_slice(content, slice))
}

fn locate_value<'content>(content: &'content str, section: &str, value: &str) -> Option<&'content str> {
    let key = format!("\"{section}\"");
    content.match_indices(key.as_str()).find_map(|(index, _)| find_node_pair(&content[index..], value))
}

fn find_node_pair<'rest>(rest: &'rest str, value: &str) -> Option<&'rest str> {
    let mut pairs = rest.match_indices("\"node\"");
    pairs.find_map(|(index, key)| quoted_value(&rest[index + key.len()..], value))
}

fn quoted_value<'tail>(tail: &'tail str, value: &str) -> Option<&'tail str> {
    let inside = tail.trim_start().strip_prefix(':')?.trim_start().strip_prefix('"')?;
    let candidate = inside.get(..value.len())?;
    (candidate == value && inside[value.len()..].starts_with('"')).then_some(candidate)
}
