use node_version_keeper::occurrence::Occurrence;
use node_version_keeper::{
    dockerfile_detector, package_json_detector, plain_version_detector, tool_versions_detector,
    workflow_detector,
};

fn texts_and_lines(occurrences: &[Occurrence]) -> Vec<(String, usize)> {
    occurrences.iter().map(|occurrence| (occurrence.text.clone(), occurrence.line)).collect()
}

fn assert_spans_match(content: &str, occurrences: &[Occurrence]) {
    for occurrence in occurrences {
        assert_eq!(&content[occurrence.start..occurrence.end], occurrence.text);
    }
}

fn expected(pairs: &[(&str, usize)]) -> Vec<(String, usize)> {
    pairs.iter().map(|(text, line)| (text.to_string(), *line)).collect()
}

#[test]
fn plain_version_file_reads_the_first_value_line() {
    let content = "# comment\n\n v20.1.0 \n22.0.0\n";
    let occurrences = plain_version_detector::find(content).unwrap();
    assert_eq!(texts_and_lines(&occurrences), expected(&[("v20.1.0", 3)]));
    assert_spans_match(content, &occurrences);
}

#[test]
fn plain_version_file_without_a_value_has_nothing() {
    assert!(plain_version_detector::find("").unwrap().is_empty());
    assert!(plain_version_detector::find("# comment only\n\n").unwrap().is_empty());
}

#[test]
fn tool_versions_reads_every_version_on_node_lines() {
    let content = "nodejs 24.11.0\npython 3.12.1\nnode 22.4.0 20.1.0 # alternate\n# nodejs 18\n\n";
    let occurrences = tool_versions_detector::find(content).unwrap();
    let pairs = expected(&[("24.11.0", 1), ("22.4.0", 3), ("20.1.0", 3)]);
    assert_eq!(texts_and_lines(&occurrences), pairs);
    assert_spans_match(content, &occurrences);
}

#[test]
fn dockerfile_reads_the_version_part_of_node_image_tags() {
    let content = concat!(
        "FROM node:24.11.0-alpine AS build\n",
        "FROM --platform=$BUILDPLATFORM docker.io/library/node:22-slim\n",
        "from node:20@sha256:abc\n",
        "FROM node:lts-alpine\n",
        "FROM node AS plain\n",
        "FROM ubuntu:22.04\n",
        "FROM mynode:24\n",
        "FROM\n",
        "RUN echo node:24\n",
        "# FROM node:18\n",
    );
    let occurrences = dockerfile_detector::find(content).unwrap();
    let pairs = expected(&[("24.11.0", 1), ("22", 2), ("20", 3), ("lts", 4)]);
    assert_eq!(texts_and_lines(&occurrences), pairs);
    assert_spans_match(content, &occurrences);
}

#[test]
fn package_json_reads_engines_and_volta_values() {
    let content = "{\n  \"name\": \"demo\",\n  \"engines\": { \"node\": \">=22\" },\n  \"volta\": { \"node\": \"20.10.0\" }\n}\n";
    let occurrences = package_json_detector::find(content).unwrap();
    assert_eq!(texts_and_lines(&occurrences), expected(&[(">=22", 3), ("20.10.0", 4)]));
    assert_spans_match(content, &occurrences);
}

#[test]
fn package_json_skips_other_node_keys_before_the_real_value() {
    let content = "{\"engines\":{\"npm\":\"1\",\"extra\":{\"node\":\"2\"},\"node\":\"24.11.0\"}}";
    let occurrences = package_json_detector::find(content).unwrap();
    assert_eq!(texts_and_lines(&occurrences), expected(&[("24.11.0", 1)]));
    assert_spans_match(content, &occurrences);
}

#[test]
fn package_json_without_a_string_node_value_has_nothing() {
    assert!(package_json_detector::find("{\"name\":\"demo\"}").unwrap().is_empty());
    assert!(package_json_detector::find("{\"engines\":{\"node\":22}}").unwrap().is_empty());
    assert!(package_json_detector::find("[1, 2]").unwrap().is_empty());
}

#[test]
fn package_json_skips_values_written_with_escapes() {
    let content = "{\"engines\":{\"node\":\"\\u0032\\u0034\"}}";
    assert!(package_json_detector::find(content).unwrap().is_empty());
}

#[test]
fn package_json_that_is_not_json_is_an_error() {
    assert!(package_json_detector::find("{ broken file").is_err());
}

#[test]
fn workflow_reads_scalars_arrays_and_block_lists() {
    let content = concat!(
        "jobs:\n",
        "  build:\n",
        "    strategy:\n",
        "      matrix:\n",
        "        node-version: [18, \"20.x\", '22.4.0'] # comment\n",
        "    steps:\n",
        "      - uses: actions/setup-node@v4\n",
        "        with:\n",
        "          node-version: 24.11.0\n",
        "      - uses: actions/setup-node@v4\n",
        "        with:\n",
        "          node-version-file: .nvmrc\n",
        "      - uses: actions/setup-node@v4\n",
        "        with:\n",
        "          node-version: ${{ matrix.node }}\n",
        "      - uses: actions/setup-node@v4\n",
        "        with:\n",
        "          node-version:\n",
        "            - 20.1.0\n",
        "            - \"22\"\n",
        "          other: x\n",
        "      - node-version: v21.0.0\n",
    );
    let occurrences = workflow_detector::find(content).unwrap();
    let pairs = expected(&[
        ("18", 5),
        ("20.x", 5),
        ("22.4.0", 5),
        ("24.11.0", 9),
        ("20.1.0", 19),
        ("22", 20),
        ("v21.0.0", 22),
    ]);
    assert_eq!(texts_and_lines(&occurrences), pairs);
    assert_spans_match(content, &occurrences);
}

#[test]
fn workflow_without_node_version_has_nothing() {
    assert!(workflow_detector::find("name: ci\non: push\n").unwrap().is_empty());
}

#[test]
fn package_json_skips_node_keys_that_are_not_the_matching_string_value() {
    let described = "{\"engines\":{\"description\":\"node\",\"node\":\"24.11.0\"}}";
    assert_eq!(package_json_detector::find(described).unwrap()[0].text, "24.11.0");
    let numbered = "{\"engines\":{\"extra\":{\"node\":1},\"node\":\"24.11.0\"}}";
    assert_eq!(package_json_detector::find(numbered).unwrap()[0].text, "24.11.0");
    let multibyte = "{\"engines\":{\"extra\":{\"node\":\"é\"},\"node\":\"2\"}}";
    assert_eq!(package_json_detector::find(multibyte).unwrap()[0].text, "2");
}
