use crate::occurrence::Occurrence;

// The `node-version` value of a workflow. Reads a single value, a one-line array like `[20, 22]`, or a multi-line list
pub fn find(content: &str) -> Result<Vec<Occurrence>, String> {
    let lines: Vec<&str> = content.lines().collect();
    let occurrences = (0..lines.len()).flat_map(|index| values_at(content, &lines, index));
    Ok(occurrences.collect())
}

fn values_at(content: &str, lines: &[&str], index: usize) -> Vec<Occurrence> {
    match key_value(lines[index]) {
        None => Vec::new(),
        Some(raw) => values_of(content, &lines[index + 1..], raw),
    }
}

fn values_of<'content>(
    content: &'content str,
    following: &[&'content str],
    raw: &'content str,
) -> Vec<Occurrence> {
    if raw.is_empty() {
        return block_list_values(content, following);
    }
    inline_values(content, raw)
}

fn key_value(line: &str) -> Option<&str> {
    let entry = line.trim_start().trim_start_matches("- ").trim_start();
    let rest = entry.strip_prefix("node-version:")?;
    Some(strip_comment(rest).trim())
}

fn inline_values(content: &str, raw: &str) -> Vec<Occurrence> {
    let inner = raw.trim_start_matches('[').trim_end_matches(']');
    let values = inner.split(',').filter_map(clean_value);
    values.map(|value| Occurrence::from_slice(content, value)).collect()
}

fn block_list_values<'content>(content: &'content str, following: &[&'content str]) -> Vec<Occurrence> {
    let items = following.iter().map_while(|line| line.trim_start().strip_prefix("- "));
    let values = items.filter_map(clean_value);
    values.map(|value| Occurrence::from_slice(content, value)).collect()
}

// The value with quotes and comments stripped. Empty values and `${{ }}` expressions are not treated as values
fn clean_value(raw: &str) -> Option<&str> {
    let trimmed = strip_comment(raw).trim();
    let unquoted = trimmed.trim_matches(|character| character == '"' || character == '\'');
    (!unquoted.is_empty() && !unquoted.contains("${{")).then_some(unquoted)
}

fn strip_comment(text: &str) -> &str {
    text.split_once(" #").map_or(text, |(before, _)| before)
}
