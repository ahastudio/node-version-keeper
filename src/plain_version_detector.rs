use crate::occurrence::Occurrence;

// `.nvmrc` and `.node-version`. The first line after skipping comments and blank lines is the version
pub fn find(content: &str) -> Result<Vec<Occurrence>, String> {
    let first_value = content.lines().map(str::trim).find(|line| is_value(line));
    let occurrence = first_value.map(|value| Occurrence::from_slice(content, value));
    Ok(occurrence.into_iter().collect())
}

fn is_value(line: &str) -> bool {
    !line.is_empty() && !line.starts_with('#')
}
