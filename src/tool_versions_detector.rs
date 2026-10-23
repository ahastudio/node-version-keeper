use crate::occurrence::Occurrence;

// Versions on the `nodejs` (or `node`) lines of `.tool-versions`
pub fn find(content: &str) -> Result<Vec<Occurrence>, String> {
    let occurrences = content.lines().flat_map(|line| line_versions(content, line));
    Ok(occurrences.collect())
}

fn line_versions(content: &str, line: &str) -> Vec<Occurrence> {
    let mut tokens = strip_comment(line).split_whitespace();
    if !matches!(tokens.next(), Some("nodejs" | "node")) {
        return Vec::new();
    }
    tokens.map(|token| Occurrence::from_slice(content, token)).collect()
}

fn strip_comment(line: &str) -> &str {
    line.split_once('#').map_or(line, |(before, _)| before)
}
