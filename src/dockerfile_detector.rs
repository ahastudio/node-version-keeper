use crate::occurrence::Occurrence;

// The version part of the tag (the leading part, without a suffix like `-alpine`) on a `FROM node:<tag>` line
pub fn find(content: &str) -> Result<Vec<Occurrence>, String> {
    let occurrences = content.lines().filter_map(|line| tag_version(content, line));
    Ok(occurrences.collect())
}

fn tag_version(content: &str, line: &str) -> Option<Occurrence> {
    let tag = node_image_tag(image_of_from_line(line)?)?;
    Some(Occurrence::from_slice(content, version_part(tag)))
}

fn image_of_from_line(line: &str) -> Option<&str> {
    let mut words = line.split_whitespace();
    words.next().filter(|word| word.eq_ignore_ascii_case("FROM"))?;
    words.find(|word| !word.starts_with("--"))
}

fn node_image_tag(image: &str) -> Option<&str> {
    let (name, tag) = image.split_once(':')?;
    (name == "node" || name.ends_with("/node")).then_some(tag)
}

fn version_part(tag: &str) -> &str {
    let end = tag.find(['-', '@']).unwrap_or(tag.len());
    &tag[..end]
}
