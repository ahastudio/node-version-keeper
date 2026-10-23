// Where a version is written in a file. start and end are byte offsets in the file content
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Occurrence {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub text: String,
}

impl Occurrence {
    // slice must be part of content. The offset is the difference between the two pointers
    pub fn from_slice(content: &str, slice: &str) -> Occurrence {
        let start = offset_within(content, slice);
        let line = content[..start].matches('\n').count() + 1;
        Occurrence { start, end: start + slice.len(), line, text: slice.to_string() }
    }
}

fn offset_within(content: &str, slice: &str) -> usize {
    slice.as_ptr() as usize - content.as_ptr() as usize
}

pub type Finder = fn(&str) -> Result<Vec<Occurrence>, String>;
